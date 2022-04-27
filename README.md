# HARM-Rust

This project contains 2 sub-projects :

* harm-rw: Off-device analysis and rewriting tool
* Secure Runtime: On-device code randomizer implemented in Rust 

## General Setup

`harm-rw` is implemented in python3 (3.6). Make sure python3 and python3-venv is installed on system. `harm-rw` depends on [capstone](https://github.com/aquynh/capstone) and [keystone](https://github.com/keystone-engine/keystone).

#### Requirements for target binary

The target binary

* must be compiled as ARM thumb2 code for ARM Cortex-M series.
* must contain symbols (i.e., not stripped) and relocations (i.e., linked with `-Wl,-q` flag).
  
#### Command line helper

The `harm-rw` tool also have command line help which describes all the options, and may be accessed with `-h`.
To start with use `harm-rw` command:

```bash
(harm) $ harm-rw --help
```

## harm-rw

### Setup

Run `setup.sh`:

```bash
$ ./setup.sh
```
  
Activate the virtualenv (from `python` directory of the repository):

```bash
$ source harm/bin/activate
```

### Usage

`samples` directory contains some sample firmware ELF images.

Example, create an instrumented version of `qsort`:

```bash
(harm) $ ./harm-rw -c samples/secure_lib/libnsclib.o -i samples/qsort.axf -p /path/to/metadata -o qsort.bin -e 0x20000
```

### Troubleshooting

A bug exists in keystone core library causes failure when recompile the binary. Please copy `python/patches/libkeystone.so` to the virtual environment:

```bash
$ cp python/patches/libkeystone.so python/harm/lib/python3.8/site-packages/keystone 
```

## Secure Runtime (Rust Prototype)

### Hardware Requirement

- NXP LPC55S69 Development Board [[Link]](https://https://www.nxp.com/design/development-boards/lpcxpresso-boards/lpcxpresso55s69-development-board:LPC55S69-EVK)
- SEGGER J-Link [[Link]](https://www.segger.com/products/debug-probes/j-link/)

### Software Requirement

- JLinkExe: Flash firmware to the target board [[Link]](https://www.segger.com/downloads/jlink/)

### Environment Setup

Add toolchain for ARMv8-M

```bash
$ rustup target add thumbv8m.main-none-eabi
```

### How To Use

1. Rewrite your firmware with `harm-rw`.
2. Copy the generated metadata YAML files to `metadata` directory.
3. Build the seure runtime
   
```bash
$ cargo objdopy --release -- -O binary demo.bin  # demo.bin is the binary of the secure runtim
```
4. Flash the secure runtime binary and the rewritten target firmware binary to LPC55S69 with SEGGER J-Link

```bash
# Download the secure runtime and target firmware to LPC55S69
# NOTE: replace XXXXX in the command line with your J-Link
# Path of the secure runtime binary and firmware is included in the J-Link script (script.jlink and script_ns.jlink), please replace with yours

# Download the secure runtime to LPC55S69
$ /path/to/JLinkExe -if SWD -speed auto -commanderscript ./script.jlink -device LPC55S69_M33_0 -SelectEmuBySN XXXXX

# Download the target firmware to LPC55S69
$ /path/to/JLikExe -if SWD -speed auto -commanderscript ./script_ns.jlink -device LPC55S69_M33_0 -SelectEmuBySN XXXXX
```

### Limitations

- Due to the poor support of TrustZone provided by `lpc55-hal` crate, we copied the HAL C code from NXP SDK and invoked via unsafe rust.
- This work is still in progress.  

## Publication

This work has been accepted by 7th IEEE European Security and Privacy (Euro S&P'22).

  - HARM: Hardware-assisted Continuous Re-randomization for Microcontrollers
