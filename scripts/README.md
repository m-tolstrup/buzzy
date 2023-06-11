# Scripts

Different Python scripts used to set up different eBPF utilities in the kernel.
The scripts used to perform eBPF operations, or similar, are written using BCC; a Python package for eBPF.

## Install Guide

Install guide for BCC found [here](https://github.com/iovisor/bcc/blob/master/INSTALL.md).

## Scripts Overview

- `translate.py` Translate ELF bytecode into a rBPF program containing a disassembler. The disassembler will print human readable instructions.
