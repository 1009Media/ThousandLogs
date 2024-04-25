# AutoTCLog-RS
Writing timecode logs manually is tedious and time consuming. AutoTCLog gets the worst of it done fast so you only have to fill in the easy parts.

This is a Rust port of a program I originally wrote in Python, from now on the old Python version is deprecated, and this is the one I'll update. The old Python code is available [here](https://github.com/sykesgabri/AutoTCLog).

## How to install:
Precompiled binaries for 64 bit Windows, MacOS on Apple Silicon, and 64 bit Linux, are available in this repo's [releases](https://github.com/sykesgabri/AutoTCLog-RS/releases). Here's how to run them on each OS:

Windows (64 bit):
- Download `autotclog-rs-X.X.X-win64.exe` from the releases page, and double click it.

MacOS (Apple Silicon):
NOTE ON MACOS VERSION: MacOS is having weird permission issues and I can't figure out how to make the program actually read folders cause MacOS won't let it.
- Download `autotclog-rs-X.X.X-macos-applesilicon` from the releases page, and double click it.

Linux (64 bit):
- Download `autotclog-rs-X.X.X-linux` from the releases page.
- Open a terminal and cd to the directory where you downloaded the program.
- Type `./autotclog-rs-X.X.X-linux` to run the program.

## Building from source:
If you need the program on a platform not in the releases, such as 32 bit Windows or an Intel Mac, follow these instructions:
- On the same device that you want to run the program on, install rustup by following the instructions found [here](https://www.rust-lang.org/tools/install).
- Install git from [here](https://git-scm.com/downloads).
- Open a terminal or command prompt and type `git clone https://github.com/sykesgabri/AutoTCLog-RS`.
- When that finishes, type `cd AutoTCLog-RS`.
- Now type `cargo build --release`, this will compile the program directly from its source code.
- When that finishes, there should be a new folder called "target", and inside, another folder called "release". The compiled binary will be in there, and it will be named "autotclog-rs" or "autotclog-rs.exe" depending on the platform you compiled it on. You can move this binary wherever you want, and can run it the same way you would run the precompiled releases.

<a rel="license" href="http://creativecommons.org/licenses/by-nc-sa/4.0/"><img alt="Creative Commons License" style="border-width:0" src="https://i.creativecommons.org/l/by-nc-sa/4.0/88x31.png" /></a><br />This work is licensed under a <a rel="license" href="http://creativecommons.org/licenses/by-nc-sa/4.0/">Creative Commons Attribution-NonCommercial-ShareAlike 4.0 International License</a>.
