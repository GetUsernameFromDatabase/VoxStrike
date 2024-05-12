# VoxStrike

Voice activated macros made in [rust](https://www.rust-lang.org/).

Does not work in-game -- I suspect [nProtect GameGuard](https://en.wikipedia.org/wiki/NProtect_GameGuard) is not allowing for keypresses in Helldivers 2.

There are a lot of firsts with this project:

- first time using [rust](https://www.rust-lang.org/)
- first time using [whisper ai](https://github.com/ggerganov/whisper.cpp)
- first time handling audio data

Currently development has been only done on windows so most likely won't work on linux or other operation systems.
> Goal is to at some point make it cross-platform.

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
  - this currently requires rust nightly, which can be installed with `rustup install nightly`
- [Download and install your preferred Whisper models](https://github.com/ggerganov/whisper.cpp/blob/master/models/README.md)
  - development has been done using `base.en` model

> accuracy has been really good with `medium.en` but ~12 second wait time is not applicable for this use case

## Building

Follow [whisper-rs build guide](https://github.com/tazz4843/whisper-rs/blob/master/BUILDING.md)
> _Path for CMAKE could be similar to `C:\Program Files\Microsoft Visual Studio\2022\Community\Common7\IDE\CommonExtensions\Microsoft\CMake\CMake\bin`_

### Windows Specific

Make sure you have toolkits installed required by [winres](https://github.com/mxre/winres?tab=readme-ov-file#toolkit).

Since this application requires [uiAccess](https://learn.microsoft.com/en-us/previous-versions/bb756929(v=msdn.10)?redirectedfrom=MSDN#uiaccess-values) the application should be run under "Program Files".

Install `signtool`, it is part of [Windows SDK](https://developer.microsoft.com/en-us/windows/downloads/windows-sdk/).\
If you have Visual Studio installed, you might already have it installed, just not on path env variables
-- `C:\Program Files (x86)\Windows Kits\10\App Certification Kit`

Whatever certificate used by `signtool` must also be trusted by the system
> this can be done my installing the certificate when viewing the `Digital Signatures` under .exe properties

Currently I do not have a way to sign this application with an appropriate certificate.
> should probably look into [MakeCert](https://learn.microsoft.com/en-us/windows/win32/seccrypto/makecert)

---

Then again winres and certificate part could be removed
as I only dabbled in it in hopes that it would allow me to send inputs to Helldivers 2.

[enigo](https://github.com/enigo-rs/enigo) notifies me with
`simulating input failed: (not all input events were sent. they may have been blocked by UIPI)`
> enigo is not included in the `cargo.toml`, I was just messing about with different libraries for keyboard inputs

### Common

Then build with

```bash
cargo build --release
```

---

To build to `output` folder [build.sh] can be used.
This builds the release version of project and copies the following there

- `profiles/` (the entire folder)
- `log4rs.yml`
- `ggml-base.en.bin` (whisper model).

## Running

You can run this application with:

```bash
cargo +nightly run
```

---

To get information about available command arguments you can do the following:

```bash
cargo +nightly run --release -- --help
```

or by running the binary with `--help` argument.

Example for windows:

```bash
vox-strike.exe --help
```
