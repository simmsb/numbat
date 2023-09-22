# Installation

## Linux

### Try immediately

On Linux, you can call

``` bash
curl -sSf https://numbat.dev/try.sh | sh
```

to try it immediately (unpacks to `/tmp` and executes from there).

### Manual installation

For a proper installation, download the latest release
[here](https://github.com/sharkdp/numbat/releases) for `x86-64` architecture.

### Arch Linux

In Arch Linux and Arch based distributions, you can install Numbat from the [AUR](https://aur.archlinux.org/packages/numbat-bin). So far, this AUR package only supports the `x86-64` (AMD64) architecture.

```bash
yay -S numbat-bin
```
For other architectures, install from source (read below).

## From source

Clone the Git repository, and build Numbat with `cargo`:

``` bash
git clone https://github.com/sharkdp/numbat
cd numbat/
cargo install -f --path numbat-cli
```

Or install the latest release using

``` bash
cargo install numbat-cli
```