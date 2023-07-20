# SCU (System Check Utility)
All-In-One fetch tool for linux-based operating systems written in [Rust](https://www.rust-lang.org/)

## Compilation
- Install rust with instruction from [rust official website](https://www.rust-lang.org/tools/install)
- Run this command in SCU directory
  ```
  cargo build --release
  ```

## How to run
- Run this command in SCU directory
  ```
  cargo run
  ```

### How it looks like
```
┌──────────System───────────┐
│ Hostname:           artix │
│ Username:              me │
│ Distro:       Artix Linux │
│ Kernel:    6.4.2-artix1-1 │
│ ┗Version:           6.4.2 │
│ Init system:       OpenRC │
│ Terminal:           Kitty │
│ Shell:               bash │
│ Uptime:        1H 33M 51S │
├─────────Processor─────────┤
│ Vendor:             Intel │
│ Model:          i7-2630QM │
│ Max freq:          2.9GHz │
│ Cores:                  4 │
│ Threads:                8 │
├─────────Packages──────────┤
│ (flatpak):             12 │
│ (pacman):             555 │
├──────────Memory───────────┤
│ Total:           15947MiB │
│ Used:             1418MiB │
├──────────Drives───────────┤
│ M4-CT256M4SSD2: 244198MiB │
└───────────────────────────┘
```
