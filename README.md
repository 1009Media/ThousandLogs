# AutoTCLog-RS
Writing timecode logs manually is tedious and time consuming. AutoTCLog gets the worst of it done fast so you only have to fill in the easy parts.

This is a Rust port of a program Gabriel Sykes originally wrote in Python, from now on the old Python version is deprecated, and this is the one that will be updated. The old Python code is available [here](https://github.com/sykesgabri/AutoTCLog).

## How to install:

**IMPORTANT:** Do not skip the steps to install FFmpeg, this program will not work without it.

Precompiled portable binaries for 64 bit Windows, MacOS on Apple Silicon, and 64 bit Linux, are available in this repo's [releases](https://github.com/sykesgabri/AutoTCLog-RS/releases). Here's how to run them on each OS:

Windows (64 bit):

- Download the latest `ffmpeg-release-full` build from [here](https://www.gyan.dev/ffmpeg/builds).
- Extract the file using WinRAR or 7zip.
- Add the contents of the `bin` folder to your system's PATH environment variable, or just copy them all to `C:\Windows`.
- Download `autotclog-rs-X_X_X-windows-PORTABLE.exe` from the releases page, and double click it.
- If Windows Defender gives you a warning prompt, click `more information` and `Run Anyway`.

MacOS (Apple Silicon):

- Install Homebrew by following the instructions [here](https://brew.sh).
- Open a terminal and type `brew install ffmpeg`.
- Download `autotclog-rs-X_X_X-macos-PORTABLE.app.zip` from the releases page.
- Unzip the downloaded file.
- Double click `autotclog-rs-X_X_X-macos-PORTABLE.app`.
- You'll probably get a warning about an untrusted developer, forcing you to go into privacy settings to allow the program to run, [this is Apple trying to extort developers by making them pay to remove this message](https://developer.apple.com/support/compare-memberships/), fuck the cunts.

Linux (64 bit):

- Install the ffmpeg package with your distro's package manager.
- Download `autotclog-rs-X_X_X-linux-PORTABLE` from the releases page.
- Open a terminal and cd to the directory where you downloaded the program.
- Type `sudo chmod +x autotclog-rs-X_X_X-linux-PORTABLE` to make the program executable.
- Type `./autotclog-rs-X_X_X-linux-PORTABLE` to run the program.

## Building from source:

If you need the program on a platform not in the releases, such as 32 bit Windows or Intel MacOS, follow these instructions:

- On the same device that you want to run the program on, install rustup by following the instructions found [here](https://www.rust-lang.org/tools/install).
- Install git from [here](https://git-scm.com/downloads).
- Install the [Tauri Prerequisites](https://tauri.app/v1/guides/getting-started/prerequisites/).
- Open a terminal or command prompt and type `cargo install tauri-cli`.
- After that finishes, type `git clone https://github.com/1009Media/AutoTCLog-RS`.
- When that finishes, type `cd AutoTCLog-RS`.
- Now type `cargo tauri build`, this will compile the program directly from its source code.
- When that finishes, there should be a new folder called "target", and inside, another folder called "release". Inside that folder (on Windows and Linux) will be a file called "AutoTCLog-RS" or "AutoTCLog-RS.exe", this is the portable version of the program.
- Inside the "bundle" folder you will find the various non-portable installers for Windows, MacOS, and Linux. Note for Mac users, the .app file you find here is your portable file, the .dmg is your permanent installer.

## CONTRIBUTORS:

- Gabriel Sykes

---
This project is licensed under the GNU LGPL-2.1
