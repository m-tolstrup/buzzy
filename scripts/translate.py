import os

# Run this from the script folder, or change the path here
# Path is relative from your current working directory
path = os.path.join('..', 'refuzzer', 'src', 'bin', 'translate.rs')

# Paste your bytecode here
elf_bytes = "b701 0000 2800 0000\
             631a fcff 0000 0000\
             b701 0000 0200 0000\
             631a f8ff 0000 0000\
             61a0 fcff 0000 0000\
             61a1 f8ff 0000 0000\
             0f10 0000 0000 0000\
             9500 0000 0000 0000"

write_str = "use rbpf::disassembler;\n\n"
write_str += "// cargo run --bin translate\n\n"
write_str += "fn main () {\n"
write_str += "\tlet prog = &[\n\t\t"

odd = True
accum = ""
count = 0

for b in elf_bytes:
    if b == " ":
        continue
    if odd:
        accum += str(b)
        odd = False
    else:
        if count == 8:
            write_str += "\n\t\t"
            count = 0
        count += 1
        accum += str(b)
        write_str += "0x" + accum + ", "
        odd = True
        accum = ""

write_str += "\n\t];\n\n"
write_str += "\tdisassembler::disassemble(prog);\n"
write_str += "}\n"

with open(path, 'w') as file:
    file.write(write_str)
