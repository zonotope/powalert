[![Build
Status](https://travis-ci.org/zonotope/powalert.svg?branch=master)](https://travis-ci.org/zonotope/powalert)

# powalert

Powalert is a cross-platform battery and system power status notifier. Get
notified when your computer is unplugged or plugged in, when the battery is
fully charged, or when the battery power level drops below a preset threshold.

## Install

### Arch Linux

There is an [Arch Linux Package](https://aur.archlinux.org/packages/powalert/)
in the arch user repository.

#### AUR Helper

The easiest way to install powalert on Arch Linux is with an aur helper. Any AUR
helper should work, but this example uses [yay](https://github.com/Jguer/yay):

```
yay -S powalert
```

#### makepkg

You can install using `makepkg` by following the [official
instructions](https://wiki.archlinux.org/index.php/Arch_User_Repository#Installing_packages)

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
