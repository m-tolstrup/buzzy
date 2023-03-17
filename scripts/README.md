# Scripts

Different Python scripts used to set up different eBPF utilities in the kernel.
All scripts are written using BCC; a Python package for eBPF.

## Install Guide

Install guide for BCC found [here](https://github.com/iovisor/bcc/blob/master/INSTALL.md).

## Scripts Overview

- `hello.py` Hello world eBPF program.
- `hello_map.py` Test eBPF map functionality
- `init_map.py` Initialize a map and print when something is written to it
- `init_map_and_test.py` Initialize a map and print when something is written to it. A probe is added for the syscall to execve to test writing to the map.