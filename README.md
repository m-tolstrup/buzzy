# ubpf-fuzz

Master project at Aalborg University.

Main focus is to write an eBPF program generator, with the goal of being able to produce programs fast, and hopefully let some slip through eBPF verifiers.
The project does this by utilizing a fuzz harness.

## Project Structure

The project has the following structure:

- PREVAIL (Submodule) (Polynomial-Runtime eBPF Verifier using an Abstract Interpretation Layer)
  - We use the PREVAIL verifier, to verify generated eBPF programs.
- uBPF (Submodule) (User space eBPF verifier and JIT-compiler for C programs)
  - Currently not used, but the intention is to use the user space JIT-compiler.
- rbpf (Submodule) (Rust eBPF)
  - As our generator is written in Rust, we use the eBPF structures provided by this Rust eBPF crate.
- refuzzer
  - Our fuzzing harness utilizing a generator and rBPF assembler.

All submodules were installed/compiled by following the README.md provided by the projects.

## How to run
The PREVAIL and uBPF submodules have some required dependencies.

- Pull code for the eBPF fuzzer and its submodules
  - `git clone --recurse-submodules https://github.com/m-tolstrup/ubpf-fuzz/`
  - `git submodule update --remote rbpf`
  - `git submodule update --remote refuzzer/faerie`
  - `git submodule update --remote ubpf`
- Build the [PREVAIL verifier](https://github.com/vbpf/ebpf-verifier) submodule
  - `cmake -B build -DCMAKE_BUILD_TYPE=Release`
  - `cmake --build build`
- Build the [uBPF](https://github.com/iovisor/ubpf) submodule
  - `cmake -S . -B build -DUBPF_ENABLE_TESTS=true`
  - `cmake --build build --config Debug`
  - `make -C vm`
  - `sudo make -C vm install`
- Run the ubpf-fuzz user-space eBPF fuzzing harness 
  - `cd refuzzer`
  - `cargo +nightly fuzz run test/random`

## Trophies
ubpf-fuzz has found the following bugs:

- [Inconsistency in load instruction handling between PREVAIL and uBPF](https://github.com/microsoft/ebpf-for-windows/issues/2362)

