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

### Output format
```
/-[System]--------------------\
| Hostname:   artix           |
| Username:   me              |
| Distro:     Artix Linux     |
| Kernel:     6.4.2-artix1-1  |
| Kernel version: 6.4.2       |
| Init system: OpenRC         |
| Terminal:   Kitty           |
| Shell:      bash            |
| Uptime:     0H 34M 53S      |
|                             |
|-[Packages]------------------|
| (flatpak): 12               |
| (pacman): 555               |
|                             |
|-[Memory]--------------------|
| Total:      15947MiB        |
| Used:       853MiB          |
|                             |
|-[Drives]--------------------|
| M4-CT256M4SSD2  : 244198MiB |
|                             |
|-[CPU]-----------------------|
| Vendor:   Intel             |
| Model:    i7-2630QM         |
| Max freq: 2.9GHz            |
| Cores:    4                 |
| Threads:  8                 |
\_____________________________/
```
