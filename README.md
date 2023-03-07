# ubpf-fuzz

Master project at Aalborg University.

Main focus is to write an eBPF program generator, with the goal of being able to produce programs fast, and hopefully let some slip through eBPF verifiers.
The project does this by utilizing a fuzz harness.

## Project Structure

The project has the following structure:

- PREVAIL (Submodule) (Polynomial-Runtime eBPF Verifier using an Abstract Interpretation Layer)
  - We use the PREVAIL verifier, to verify generated eBPF programs.
- uBPF (Submodule) (User space EBPF for C programs)
  - Currently not used, but the intention is to use the user space JIT-compiler.
- rbpf (Submodule) (Rust eBPF)
  - As our generator is written in Rust, we use the eBPF structures provided by this Rust eBPF crate.
- refuzzer
  - Our fuzzing harnes utilizing a generator and rBPF assembler.

All submodules were installed/compiled by following the README.md provided by the projects.
