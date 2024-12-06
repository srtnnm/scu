<div align="center">
  
# SCU
Command-line system fetch utility written in [Rust](https://www.rust-lang.org)

### Currently supported operating systems
<img src="https://img.shields.io/badge/Linux-FCC624?style=for-the-badge&logo=linux&logoColor=black">
<img src="https://img.shields.io/badge/Android-3DDC84?style=for-the-badge&logo=android&logoColor=white">

</div>

## Screenshots
<details height="100px"><summary>Gentoo</summary>
<div>

![gentoo](images/gentoo.png)
</div>
</details>
<details height="100px"><summary>with --simplify</summary>
<div>

![manjaro](images/gentoo_simplify.png)
</div>
</details>
<details height="100px"><summary>Android</summary>
<div>

> Running in termux

![android](images/android.jpg)
</div>
</details>

## Usage

**You can download latest release of scu from [releases](https://gitlab.com/omnitix/scu/-/releases) or compile it by yourself**

### CLI flags

* `--simplify`

  Outputs information in a much simpler form, forced by default when output is piped
* `--ignore-pipe`

  Outputs information in regular form, even if it's piped (disables --simplify forcing)
* `--force-versions`

  Enables version fetching (was disabled by default in commit [a0c0bada](https://gitlab.com/omnitix/scu/-/commit/a0c0badaa2b506496558797c3a02957ece0f3ff9#9541a669da5368e41d92810535106685569e34d0_54_52) due to bad performance)
* `-v`, `--version`

  Prints scu's version

* `-h`, `--help`

  Prints help page

* `--raw-models`

  Show models (CPU, GPU, etc.) without processing

* `--multicpu`

  Show multiple cpus instead of single cpu (currently unstable!)

### Configuration

scu generates default config at `~/.config/omnid/scu` when you first start it.
Config format is ordered scu features, separated by commas, the default is `system,processor,graphics,memory,packages,drives,battery`.
Entries that doesn't exists will be ignored.

## Compilation

1. Install Rust toolchain.
2. Clone scu and compile it with cargo.

``` shell
$ git clone https://gitlab.com/omnitix/scu
$ cd scu
$ cargo build --release
```

Compiled binary is located at `target/release/scu`.

3. Install systemwide (optional)
```
$ cp target/release/scu /usr/local/bin
```