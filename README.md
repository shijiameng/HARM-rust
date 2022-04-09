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
(harm) $ ./harm-rw -c secure_service_CMSE_lib.o -i samples/qsort.axf -p /path/to/metadata -o qsort.bin -e 0x20000
```

### Troubleshooting

A bug exists in keystone core library causes failure when recompile the binary. Please copy `python/patches/libkeystone.so` to the virtual environment:

```bash
$ cp python/patches/libkeystone.so python/harm/lib/python3.8/site-packages/keystone 
```

## Secure Runtime (Rust Prototype)

Secure Runtime core has been implemented.