# buzzy

This project was developed as a part of a master thesis at Aalborg University. The project focused on fuzzing eBPF technologies.
We targeted the PREVAIL verifier and uBPF virtual machine, to find conformance issues between the two. The main bulk of code written for the project consists of an eBPF program generator, and a parser to produce ELF files.

We will make the project report available when we have defended the thesis.

## Project Structure

The Buzzy project has the following folder structure:

- **PREVAIL** (Submodule) (Polynomial-Runtime eBPF Verifier using an Abstract Interpretation Layer)
  - We use the PREVAIL verifier, to verify generated eBPF programs.
- **uBPF** (Submodule) (User space eBPF verifier and JIT-compiler for C programs)
  - We use the uBPF virtual machine to run the verified programs.
- **rbpf** (Submodule) (Rust eBPF)
  - We use the eBPF structures provided by this Rust eBPF crate to generate eBPF programs.
- **buzzy**
  - Our fuzzing harness which utilizes a generator and ELF file parser.
- **buzzy/faerie**
  - Used to parse generated programs and produce ELF files.
- **scripts**
  - Scripts used for experiments for the report and quick bug/error overviews


We used buzzy to target the PREVAIL verifier and uBPF virtual machine, but buzzy should be applicable to other eBPF technolgies, without much implementation overhead. In the future, PREVAIL and uBPF might be removed as submodules.

All submodules were installed/compiled by following the README.md provided by the projects.

## How to run
The PREVAIL and uBPF submodules have some required dependencies.

- Pull code for the eBPF fuzzer and its submodules
  - `git clone --recurse-submodules https://github.com/m-tolstrup/buzzy/`
  - `git submodule update --remote rbpf`
  - `git submodule update --remote buzzy/faerie`
  - `git submodule update --remote ubpf`
  - `git submodule update --remote ebpf-verifier`
- Build the [PREVAIL verifier](https://github.com/vbpf/ebpf-verifier) submodule
  - `cmake -B build -DCMAKE_BUILD_TYPE=Release`
  - `cmake --build build`
- Build the [uBPF](https://github.com/iovisor/ubpf) submodule
  - `cmake -S . -B build -DUBPF_ENABLE_TESTS=true`
  - `cmake --build build --config Debug`
  - `make -C vm`
  - `sudo make -C vm install`
- Run the buzzy user-space eBPF fuzzing harness 
  - `cd buzzy`
  - `cargo +nightly fuzz run test/random/stack_sequence/random_maps`

## Trophies
buzzy has found the following (confirmed) bugs:

- [Inconsistency in load instruction handling between PREVAIL and uBPF](https://github.com/vbpf/ebpf-verifier/issues/484)
- [Segmentation fault (core dumped) - Null value in modulo operations](https://github.com/vbpf/ebpf-verifier/issues/493)

## Future Work

We see the following areas as next the additions or changes for buzzy:

- **Move away from cargo-fuzz**: cargo-fuzz only ended up being used for what is essentially a looping mechanism. Implementing a version of this that would better suit buzzy would be greatly beneficial for the project.
- **Guided fuzzing**: Extending above, this setup could allow for better guided fuzzing.
- **Extendned fault detection**: For the RPEVAIL/uBPF setup, buzzy only observed for conformance issues between the targets. No run time behavior or similar is captured.
- **Bug taming**: Some bugs are generated often. Mechanisms to handle this would be useful.
- **More strategies**: We think that strategies revolving around program type and context is the next step for strategies.


