import os

# Run this from the script folder, or change the path here
# Path is relative from your current working directory
path = os.path.join('..', 'refuzzer', 'src', 'bin', 'translate.rs')

# Paste your bytecode here
elf_bytes = "b702 0000 0000 0000\
             7b2a f0ff 0000 0000\
             bf21 0000 0000 0000\
             8510 0000 0100 0000\
             79a3 f0ff 0000 0000\
             b702 0000 0a00 0000\
             7b2a d0ff 0000 0000\
             1804 0000 ffff ff00\
             7b4a d8ff 0000 0000\
             bf31 0000 0000 0000\
             8510 0000 0100 0000\
             79a2 d0ff 0000 0000\
             b701 0000 0200 0000\
             7b1a e0ff 0000 0000\
             8510 0000 0100 0000\
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
