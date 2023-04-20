import os
import sys
import subprocess


if sys.argv:
    o_path = os.path.join(*sys.argv[1].split("/"))
    instr_count = int(sys.argv[2])
else:
    # Set path of the .o file
    o_path = os.path.join("..", "refuzzer", "logs", "error1681822839119.o")
    # Set the number of instructions to read here
    instr_count = 4

# Run this from the script folder, or change the path here
# Path is relative from your current working directory
path = os.path.join('..', 'refuzzer', 'src', 'bin', 'translate.rs')

elf_bytes = b""
with open(o_path, 'rb') as file:
    file.read(64)
    elf_bytes += file.read(instr_count * 8)

write_str = "use rbpf::disassembler;\n\n"
write_str += "// cargo run --bin translate\n\n"
write_str += "fn main () {\n"
write_str += "\tlet prog = &[\n\t\t"

count = 0
for b in elf_bytes:
    count += 1
    write_str += hex(b) + ", "
    if count == 8:
        write_str += "\n"
        count = 0

write_str += "\n\t];\n\n"
write_str += "\tdisassembler::disassemble(prog);\n"
write_str += "}\n"

with open(path, 'w') as file:
    file.write(write_str)

try:
    cmd_str = "cargo run --bin translate"
    path = os.path.join("..", "refuzzer")
    os.chdir(path)
    subprocess.run(cmd_str, shell=True)
except:
    print("Exception during 'cargo run --bin translate'")
