# SCU (System Check Utility)
All-In-One fetch tool for linux-based operating systems written in [Rust](https://www.rust-lang.org/)


# Compiling
1. Install cargo
> Simply run this command (taken from [official documentation](https://doc.rust-lang.org/cargo/getting-started/installation.html))
```
curl https://sh.rustup.rs -sSf | sh
```
2. Run install.sh (that script will build and install SCU)
```
bash install.sh
```
If you don't want to install to your system, just run
```
cargo build --release
```
binary file will be stored in target/release


# Usage
Just run using:
```
scu
```
