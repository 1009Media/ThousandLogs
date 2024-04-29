# AutoTCLog-RS
Writing timecode logs manually is tedious and time consuming. AutoTCLog gets the worst of it done fast so you only have to fill in the easy parts.

This is a Rust port of a program I originally wrote in Python, from now on the old Python version is deprecated, and this is the one I'll update. The old Python code is available [here](https://github.com/sykesgabri/AutoTCLog).

## How to install:

**IMPORTANT:** Do not skip the steps to install FFmpeg, this program will not work without it.

Precompiled binaries for 64 bit Windows, MacOS on Apple Silicon, and 64 bit Linux, are available in this repo's [releases](https://github.com/sykesgabri/AutoTCLog-RS/releases). Here's how to run them on each OS:

Windows (64 bit):

- Download the latest `ffmpeg-release-full` build from [here](https://www.gyan.dev/ffmpeg/builds).
- Extract the file using WinRAR or 7zip.
- Add the contents of the `bin` folder to your system's PATH environment variable, or just copy them all to `C:\Windows`.
- Download `autotclog-rs-X_X_X-windows.exe` from the releases page, and double click it.
- If Windows Defender gives you a warning prompt, click `more information` and `Run Anyway`.

MacOS (Apple Silicon):

- Install Homebrew by following the instructions [here](https://brew.sh).
- Open a terminal and type `brew install ffmpeg`.
- Download `autotclog-rs-X_X_X-macos` from the releases page.
- Open a terminal and cd to the directory where you downloaded the program.
- Type `sudo chmod +x autotclog-rs-X_X_X-macos` to make the program executable.
- Type `./autotclog-rs-X_X_X-macos` to run the program.

Linux (64 bit):

- Install the ffmpeg package with your distro's package manager.
- Download `autotclog-rs-X_X_X-linux` from the releases page.
- Open a terminal and cd to the directory where you downloaded the program.
- Type `sudo chmod +x autotclog-rs-X_X_X-linux` to make the program executable.
- Type `./autotclog-rs-X_X_X-linux` to run the program.

## Building from source:

If you need the program on a platform not in the releases, such as 32 bit Windows or an Intel Mac, follow these instructions:

- On the same device that you want to run the program on, install rustup by following the instructions found [here](https://www.rust-lang.org/tools/install).
- Install git from [here](https://git-scm.com/downloads).
- Open a terminal or command prompt and type `git clone https://github.com/sykesgabri/AutoTCLog-RS`.
- When that finishes, type `cd AutoTCLog-RS`.
- Now type `cargo build --release`, this will compile the program directly from its source code.
- When that finishes, there should be a new folder called "target", and inside, another folder called "release". The compiled binary will be in there, and it will be named "autotclog-rs" or "autotclog-rs.exe" depending on the platform you compiled it on. You can move this binary wherever you want, and can run it the same way you would run the precompiled releases.

<a rel="license" href="http://creativecommons.org/licenses/by-nc-sa/4.0/"><img alt="Creative Commons License" style="border-width:0" src="https://i.creativecommons.org/l/by-nc-sa/4.0/88x31.png" /></a><br />This work is licensed under a <a rel="license" href="http://creativecommons.org/licenses/by-nc-sa/4.0/">Creative Commons Attribution-NonCommercial-ShareAlike 4.0 International License</a>.
