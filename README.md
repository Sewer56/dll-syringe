# dll-syringe

[![CI](https://github.com/OpenByteDev/dll-syringe/actions/workflows/ci.yml/badge.svg)](https://github.com/OpenByteDev/dll-syringe/actions/workflows/ci.yml)
[![crates.io](https://img.shields.io/crates/v/dll-syringe.svg)](https://crates.io/crates/dll-syringe)
[![Documentation](https://docs.rs/dll-syringe/badge.svg)](https://docs.rs/dll-syringe)
[![dependency status](https://deps.rs/repo/github/openbytedev/dll-syringe/status.svg)](https://deps.rs/repo/github/openbytedev/dll-syringe)
[![MIT](https://img.shields.io/crates/l/dll-syringe.svg)](https://github.com/OpenByteDev/dll-syringe/blob/master/LICENSE)

A windows dll injection library written in Rust.

## Supported scenarios

| Injector Process | Target Process | Supported?                                 |
| ---------------- | -------------- | ------------------------------------------ |
| 32-bit           | 32-bit         | Yes                                        |
| 32-bit           | 64-bit         | No                                         |
| 64-bit           | 32-bit         | Yes (requires feature `into-x86-from-x64`) |
| 64-bit           | 64-bit         | Yes                                        |

## Usage
### Inject & Eject
This crate allows you to inject and eject a DLL into a target process.
The example below will inject and then eject `injection_payload.dll` into the process called "ExampleProcess".

```rust no_run
use mini_syringe::{Syringe, process::OwnedProcess};

// find target process by name
let target_process = OwnedProcess::find_first_by_name("ExampleProcess").unwrap();

// create a new syringe for the target process
let syringe = Syringe::for_process(target_process);

// inject the payload into the target process
let injected_payload = syringe.inject("injection_payload.dll").unwrap();

// do something else

// eject the payload from the target (optional)
syringe.eject(injected_payload).unwrap();
```

## License
Licensed under MIT license ([LICENSE](https://github.com/OpenByteDev/dll-syringe/blob/master/LICENSE) or http://opensource.org/licenses/MIT)

## Instructions for Contributors

### Prerequisites

You will need the nightly toolchains of Rust and Cargo to build/test this project.

```
rustup target add x86_64-pc-windows-msvc --toolchain nightly
rustup target add i686-pc-windows-msvc --toolchain nightly
```

> [!NOTE]
> Also applies to developing on Linux, you'll need it for your IDE (i.e. rust-analyzer or RustRover) to work properly.

### Run Tests

Run the `./scripts/test.ps1` script from PowerShell.

### Running Tests on Linux

You'll need `cargo xwin` to build the MSVC targets on Linux:

```
cargo install cargo-xwin
```

After that, you can run the tests with `./scripts/test-wine.ps1` PowerShell script.
(As opposed to `./scripts/test.ps1`)

Make sure you have Wine installed!

## Attribution

Stripped down fork of [dll-syringe](https://github.com/OpenByteDev/dll-syringe) from [OpenByteDev](https://github.com/OpenByteDev).
Which in turn is inspired by my own [Reloaded.Injector](https://github.com/Reloaded-Project/Reloaded.Injector).