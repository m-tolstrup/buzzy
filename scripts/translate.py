# This script translates an .o file using the rBPF crate in this project
# A path is given to the .o file, and a number of instructions
# The number of instructions is counted from the first instruction after the header bytes

import os
import sys
import subprocess

if sys.argv:
    o_path = os.path.join(*sys.argv[1].split("/"))
    instr_count = int(sys.argv[2])
else:
    # Path is set here if it is not given through command line
    o_path = os.path.join("..", "buzzy", "logs", "error1681822839119.o")
    # Set the number of instructions to read here
    instr_count = 4

# The file is saved in the buzzy cargo crate
path = os.path.join("..", "buzzy", "src", "bin", "translate.rs")

# Read bytes, but skip 64 header bytes
try:
    elf_bytes = b""
    with open(o_path, 'rb') as file:
        file.read(64)
        # Each instruction is eight bytes
        elf_bytes += file.read(instr_count * 8)
except:
    print("Failed to read from .o file")

# Rust boilerplate
write_str = "use rbpf::disassembler;\n\n"
write_str += "// cargo +nightly run --bin translate\n\n"
write_str += "fn main () {\n"
write_str += "\tlet prog = &[\n\t\t"

# Add the bytes from the .o file
count = 0
for b in elf_bytes:
    count += 1
    write_str += hex(b) + ", "
    if count == 8:
        write_str += "\n\t\t"
        count = 0

# More boilerpalte
write_str += "];\n\n"
write_str += "\tdisassembler::disassemble(prog);\n"
write_str += "}\n"

# Write it to the file
try:
    with open(path, 'w') as file:
        file.write(write_str)
except:
    print("Failed to write to .rs file")

# Run command to run the translation
try:
    cmd_str = "cargo +nightly run --bin translate"
    path = os.path.join("..", "buzzy")
    os.chdir(path)
    subprocess.run(cmd_str, shell=True)
except:
    print("Failed to run 'cargo +nightly run --bin translate'")
