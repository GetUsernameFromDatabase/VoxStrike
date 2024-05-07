# VoxStrike

Voice activated macros made in [rust](https://www.rust-lang.org/).

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

_Path for CMAKE could be similar to `C:\Program Files\Microsoft Visual Studio\2022\Community\Common7\IDE\CommonExtensions\Microsoft\CMake\CMake\bin`_

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
