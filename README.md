# Voltcraft Energy Logger 4000 Reader

Voltcraft Energy Logger 4000 Reader is a tool for processing the Voltcraft Energy Logger 4000 binary data which can be downloaded from the device using an SD card.

The code is certainly not perfect, but kept as simple as possible

## Usage

$: `EnergyLogger4000-Reader -h`

```
Usage: EnergyLogger4000-Reader [options]

Options:
    -f, --file NAME     Read file
    -d, --directory NAME
                        Read files from directory
    -h, --help          Print this help menu
    -v, --version       Output version information and exit
```

### Examples

You can load a single file

```
EnergyLogger4000-Reader -f B08F9CD2.BIN
```

Or a complete directory  
<font color="red">ATTENTION</font>: Only one complete data set from the SD card should be included at any one time

```
EnergyLogger4000-Reader -d /mnt/
```

# Known bugs

* An absolute path must always be specified. Unfortunately, a path such as `~/B08F9CD2.BIN` or `~/mypath/` does not work.

# Reference

* http://wiki.td-er.nl/index.php?title=Energy_Logger_3500