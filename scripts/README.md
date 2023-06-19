# Scripts

Different Python scripts to gather results for thesis experiments, and aggregating fuzzing results and errors.

## Scripts Overview

- `collect_exp_data.py` Get results for PREVAIL verification time and memory usage.
- `count_errors.py` Counts the errors encountered during unmarshaling in PREVAIL.
- `instr_graph.py` Create a column diagram for percentage of safe programs over program instruction count.
- `print_all_errors.py` Print all generated errors indirectly by runnign erroneous programs in uBPF.
- `print_target_output.py` Print the human readable bytecode of a program, and the output of PREVAIL and uBPF. 
- `translate.py` Translate ELF bytecode into a rBPF program containing a disassembler. The disassembler will print human readable bytecode instructions.
