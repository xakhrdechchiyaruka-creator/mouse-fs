use clap::{Parser, Subcommand};
use hidapi::HidApi;

// Logitech Unifying receiver USB identifiers
const LOGITECH_VID: u16 = 0x046d;
const RECEIVER_PID: u16 = 0xc52b;

// Device index 0x01 = first paired device on the receiver (the mouse)
const DEVICE_IDX: u8 = 0x01;

// HID++ feature index for AdjustableDPI on this specific mouse.
// Feature indices are not fixed across devices. This was discovered by
// enumerating the feature table via IFeatureSet (feature 0x0001).
const FEAT_DPI: u8 = 0x12;

#[derive(Parser)]
#[command(name = "mouse-fs")]
#[command(about = "Store 2 bytes of data in your Logitech MX Vertical's DPI register")]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// Write up to 2 bytes, e.g: mouse-fs write "hi"
    Write { data: String },
    /// Read back the stored bytes
    Read,
    /// Reset to 1000 DPI
    Reset,
    /// Show raw DPI value
    Raw,
}

/// Send a HID++ 2.0 "short" report (7 bytes) and return the response.
///
/// HID++ 2.0 package layout: [report_id, device_idx, feature_idx, fn_and_sw_id, param0, param1, param2]
///
/// report_id 0x10 = short report (7 bytes)
/// report_id 0x11 = long report (20 bytes) - blocked by macOS IOHIDManager on this device
///
/// fn_and_sw_id packs the function index (high nibble) and a software ID (low nibble).
/// Software ID 0x1 is arbitrary. It's echoed back in the response so multiple concurrent callers can match
/// replies to requests.
///
/// Returns None if the device sends no response or returns an error packet
/// (error packets have 0xff in byte[2])
fn hidpp_short(device: &hidapi::HidDevice, feat: u8, func: u8, p: [u8; 3]) -> Option<Vec<u8>> {
    let msg = [0x10, DEVICE_IDX, feat, (func << 4) | 0x01, p[0], p[1], p[2]];
    device.write(&msg).ok()?;
    let mut buf = [0u8; 20];
    match device.read_timeout(&mut buf, 2000) {
        Ok(n) if n > 0 && buf[2] != 0xff => Some(buf[..n].to_vec()),
        _ => None,
    }
}

/// Read the current DPI value from the mouse.
///
/// AdjustableDPI function 0x02 = getSensorDPI(sensor_index).
/// Sensor index 0 is the only sensor on the MX Vertical.
/// The DPI value is a big-endian u16 at bytes [5..6] of the response.
///
/// This doubles as our 2-byte storage primitive: the DPI register accepts
/// arbitrary u16 values (200-4000+ confirmed, no quantization) and persists them
/// in the mouse's flash across power cycles and reconnections.
fn dpi_read(device: &hidapi::HidDevice) -> u16 {
    hidpp_short(device, FEAT_DPI, 0x02, [0, 0, 0])
        .map(|r| ((r[5] as u16) << 8) | r[6] as u16)
        .unwrap_or(0)
}

/// Write a u16 value into the DPI register.
///
/// AdjustableDPI function 0x03 = setSensorDPI(sensor_index, dpi_hi, dpi_lo).
/// The mouse often sends no response packet on successful write. It only responds
/// when there's an error or a value change to report. Rather than treating a missing
/// response as failure, we verify by reading back.
fn dpi_write(device: &hidapi::HidDevice, dpi: u16) -> bool {
    hidpp_short(
        device,
        FEAT_DPI,
        0x03,
        [0x00, (dpi >> 8) as u8, (dpi & 0xff) as u8],
    );
    dpi_read(device) == dpi
}

/// Find and open the first Logitech Unifying receiver interface that has the mouse paired.
///
/// A single receiver exposes multiple HID interfaces (different usage pages).
/// Only one of them speaks HID++ to paired devices. We find it by pinging feature 0x0000 (IRoot) and
/// checking that the response echoes back our device index. We then confirm the DPI feature responds
/// before accepting it, since IRoot might respond on interfaces where higher features don't.
///
/// macOS assigns DevSrvsID numbers dynamically so we can't rely on a fixed path across sessions.
/// This probe approach works regardless of enumeration order.
fn open_device(api: &HidApi) -> hidapi::HidDevice {
    for info in api
        .device_list()
        .filter(|d| d.vendor_id() == LOGITECH_VID && d.product_id() == RECEIVER_PID)
    {
        let Ok(dev) = info.open_device(api) else {
            continue;
        };

        // IRoot ping: feature 0x00, function 0x01 (GetProtocolVersion)
        let msg = [0x10, DEVICE_IDX, 0x00, 0x11, 0x00, 0x00, 0x00];
        if dev.write(&msg).is_err() {
            continue;
        }

        let mut buf = [0u8; 20];
        if let Ok(n) = dev.read_timeout(&mut buf, 500) {
            if n > 0 && buf[1] == DEVICE_IDX {
                if dpi_read(&dev) != 0 {
                    return dev;
                }
            }
        }
    }
    panic!("no responsive Logitech receiver found. Is Logi Options+ running?");
}

fn main() {
    let cli = Cli::parse();
    let api = HidApi::new().expect("Failed to init HID API");
    let device = open_device(&api);

    match cli.cmd {
        Cmd::Write { data } => {
            let bytes = data.as_bytes();
            if bytes.is_empty() || bytes.len() > 2 {
                eprintln!("error: data must be 1–2 bytes (got {})", bytes.len());
                std::process::exit(1);
            }
            let hi = bytes[0];
            let lo = if bytes.len() > 1 { bytes[1] } else { 0x00 };
            let dpi = ((hi as u16) << 8) | lo as u16;

            if dpi_write(&device, dpi) {
                println!("written: {:?} → DPI {:#06x} ✓", data, dpi);
            } else {
                eprintln!("write failed");
                std::process::exit(1);
            }
        }

        Cmd::Read => {
            let dpi = dpi_read(&device);
            let hi = (dpi >> 8) as u8;
            let lo = (dpi & 0xff) as u8;
            // Show as string if both bytes are printable ASCII, otherwise hex
            let display = match (hi.is_ascii_graphic(), lo.is_ascii_graphic()) {
                (true, true) => format!("\"{}{}\"", hi as char, lo as char),
                (true, false) => format!("\"{}\" + {:#04x}", hi as char, lo),
                (false, true) => format!("{:#04x} + \"{}\"", hi, lo as char),
                (false, false) => format!("{:#04x} {:#04x}", hi, lo),
            };
            println!("stored: {} (raw DPI {:#06x})", display, dpi);
        }

        Cmd::Reset => {
            dpi_write(&device, 1000);
            println!("reset to 1000 DPI");
        }

        Cmd::Raw => {
            println!("DPI register: {}", dpi_read(&device));
        }
    }
}
