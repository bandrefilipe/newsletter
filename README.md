# Newsletter

## Environment Setup
The necessary steps needed to build/run this code locally.

### Rust Toolchain
Install `rustup` following the [official guide](https://www.rust-lang.org/tools/install "rust-lang.org").

### Faster Linking 
Install `lld` (Windows and Linux) or `zld` (macOS) to reduce compilation time.

| OS             | Command                                                                      |
|----------------|------------------------------------------------------------------------------|
| Windows        | `cargo install -f cargo-binutils && rustup component add llvm-tools-preview` |
| Linux (Debian) | `sudo apt-get install lld clang`                                             |
| Linux (Arch)   | `sudo pacman -S lld clang`                                                   |
| macOS          | `brew install michaeleisel/zld/zld`                                          |
