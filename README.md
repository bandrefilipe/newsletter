# Newsletter

## Dependencies
- [Actix Web](https://actix.rs/ "actix.rs"): A powerful, pragmatic, and extremely fast web framework for Rust.
- [Tokio](https://tokio.rs/ "tokio.rs"): A runtime for writing reliable network applications without compromising speed.
- [Serde](https://serde.rs/ "serde.rs"): A generic serialization/deserialization framework.
- [config-rs](https://docs.rs/crate/config "docs.rs"): Layered configuration system for Rust applications (with strong support for 12-factor applications).
- [uuid](https://docs.rs/crate/uuid/ "docs.rs"): Generate and parse universally unique identifiers (UUIDs).
- [chrono](https://docs.rs/crate/chrono "docs.rs"): Date and time library for Rust.

## Environment Setup
The necessary steps needed to build/run this code locally.

### Rust Toolchain
Install `rustup` following the [official guide](https://www.rust-lang.org/tools/install "rust-lang.org").

### Faster Linking
Install `lld` (Windows and Linux) or `zld` (macOS) to reduce compilation time.

| OS             | Command                                                                      |
|----------------|------------------------------------------------------------------------------|
| Linux (Arch)   | `sudo pacman -S lld clang`                                                   |
| Linux (Debian) | `sudo apt-get install lld clang`                                             |
| macOS          | `brew install michaeleisel/zld/zld`                                          |
| Windows        | `cargo install -f cargo-binutils && rustup component add llvm-tools-preview` |

### OpenSSL
Install OpenSSL, required for building the `openssl-sys` crate.

| OS             | Command                                       |
|----------------|-----------------------------------------------|
| Linux (Arch)   | `sudo pacman -S pkg-config openssl`           |
| Linux (Debian) | `sudo apt-get install pkg-config openssl-dev` |
| macOS          | `brew install openssl@1.1`                    |

### Docker
Install `docker` following the [official guide](https://docs.docker.com/engine/install/ "docs.docker.com").

It is used to run this app's database – which can be done executing the `./scripts/init_db.sh` script.
