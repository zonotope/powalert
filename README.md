[![Build
Status](https://travis-ci.org/zonotope/powalert.svg?branch=master)](https://travis-ci.org/zonotope/powalert)

# powalert

Powalert is a cross-platform battery and system power status notifier. Get
notified when your computer is unplugged or plugged in, when the battery is
fully charged, or when the battery power level drops below a preset threshold.

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
