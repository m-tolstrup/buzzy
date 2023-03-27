# Scripts

Different Python scripts used to set up different eBPF utilities in the kernel.
The scripts used to perform eBPF operations, or similar, are written using BCC; a Python package for eBPF.

## Install Guide

Install guide for BCC found [here](https://github.com/iovisor/bcc/blob/master/INSTALL.md).

## Scripts Overview

- `translate.py` Translate ELF bytecode into a rBPF program containing a disassembler. The disassembler will print human readable instructions.
- `hello.py` Hello world eBPF program.
- `hello_map.py` Test eBPF map functionality
- `init_map.py` Initialize a map and print when something is written to it
- `init_map_and_test.py` Initialize a map and print when something is written to it. A probe is added for the syscall to execve to test writing to the map.
