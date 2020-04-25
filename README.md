[![Build
Status](https://travis-ci.org/zonotope/powalert.svg?branch=master)](https://travis-ci.org/zonotope/powalert)

# powalert

Powalert is a cross-platform battery and system power status notifier. Get
notified when your computer is unplugged or plugged in, when the battery is
fully charged, or when the battery power level drops below a preset threshold.

## Install

### Arch Linux

There is an [Arch Linux Package](https://aur.archlinux.org/packages/powalert/)
in the Arch User Repository. The easiest way to install is with an aur helper.
Any AUR helper should work, but this example uses
[yay](https://github.com/Jguer/yay):

```
yay -S powalert
```

You can also install using `makepkg` by following the [official
instructions](https://wiki.archlinux.org/index.php/Arch_User_Repository#Installing_packages)

### Source

To install from source, first build the program by following these instructions:
1. Install [rust](https://www.rust-lang.org/tools/install), which includes the
   `cargo` utility, and
   [git](https://git-scm.com/book/en/v2/Getting-Started-Installing-Git) if you
   haven't already.
2. Clone the repository:
```
git clone https://github.com/zonotope/powalert.git
```
3. Change to the directory and build:
```
cd powalert
cargo build --release
```

The program will saved as `target/release/powalert`, and you can copy it
anywhere in your path to use it.

There is also a [systemd](https://www.freedesktop.org/wiki/Software/systemd/)
unit to run powalert automatically in the `systemd` directory.

## Usage

```
powalert 0.1.0
System power notifier

USAGE:
    powalert [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -i, --interval <interval>          Pause time before polling batteries for updates in seconds
    -t, --low-threshold <threshold>    Threshold percentage to send low power notifications
    -v, --verbose <verbose>            Verbosity level (either '0', '1', or '2')
```
