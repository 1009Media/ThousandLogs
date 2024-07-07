# AutoTCLog-RS
Writing timecode logs manually is tedious and time consuming. AutoTCLog gets the worst of it done fast so you only have to fill in the easy parts.

This is a Rust port of a program Gabriel Sykes originally wrote in Python, from now on the old Python version is deprecated, and this is the one that will be updated. The old Python code is available [here](https://github.com/sykesgabri/AutoTCLog).

## How to install:

**IMPORTANT:** Do not skip the steps to install FFmpeg, this program will not work without it.

Precompiled portable binaries for 64 bit Windows, MacOS on Apple Silicon, and 64 bit Linux, are available in this repo's [releases](https://github.com/sykesgabri/AutoTCLog-RS/releases). Here's direct links to the downloads, and how to run them on each OS:

| Platform                    | x86_64 Windows (10+)                                                                                                                                 | Apple Silicon macOS                                                                                                                                      | x86_64 Linux                                                                                                                             |
|-----------------------------|------------------------------------------------------------------------------------------------------------------------------------------------------|----------------------------------------------------------------------------------------------------------------------------------------------------------|------------------------------------------------------------------------------------------------------------------------------------------|
| **FFmpeg Instructions**     | Download [`ffmpeg-release-full`](https://www.gyan.dev/ffmpeg/builds)<br>Extract using WinRar or 7zip<br>Copy contents of `bin` folder to C:\Windows  | Install [Homebrew](https://brew.sh)<br>Open a terminal and type `brew install ffmpeg`                                                                    | Install the `ffmpeg` package with your distro's package manager.                                                                         |
| **AutoTCLog-RS Executable** | [AutoTCLog-RS-3_0_2-Windows-PORTABLE.exe](https://github.com/1009Media/AutoTCLog-RS/releases/download/3.0.2/AutoTCLog-RS-3_0_2-Windows-PORTABLE.exe) | [AutoTCLog-RS-3_0_2-macOS-PORTABLE.app.zip](https://github.com/1009Media/AutoTCLog-RS/releases/download/3.0.2/AutoTCLog-RS-3_0_2-macOS-PORTABLE.app.zip) | [AutoTCLog-RS-3_0_2-Linux-PORTABLE](https://github.com/1009Media/AutoTCLog-RS/releases/download/3.0.2/AutoTCLog-RS-3_0_2-Linux-PORTABLE) |

### Special considerations:

**Windows:**

- If Windows Defender gives you a warning prompt, click `More Info` then `Run Anyway`.

**macOS:**

- You'll get a warning about an untrusted developer, forcing you to go into privacy settings to allow the program to run, [if I wanted to prevent this from happening, I would have to pay Apple](https://developer.apple.com/support/compare-memberships/).

**Linux:**

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
