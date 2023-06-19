import os
import sys
import subprocess

# This script translates an .o file using the rBPF crate in this project
# The script takes two arguments:
# First: The last digits of the error file, the script matches the rest.
# Second: The number of instructions to print. Counted from the first instruction after the header bytes.

logs = os.path.join("..", "buzzy", "logs")

def main():
    if len(sys.argv) > 2:
        error_file_num = sys.argv[1]
        instr_count = sys.argv[2]

        file_ending = error_file_num + ".o"

        # The file is saved in the buzzy cargo crate
        save_path = os.path.join("..", "buzzy", "src", "bin", "translate.rs")

        for file in os.listdir(logs):
            if file.endswith(file_ending):

                error_path = os.path.join(logs, file)

                print(error_path)

                # Read bytes, but skip 64 header bytes
                try:
                    elf_bytes = b""
                    with open(error_path, 'rb') as f:
                        f.read(64)
                        # Each instruction is eight bytes
                        elf_bytes += f.read(int(instr_count) * 8)
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
                    with open(save_path, 'w') as f:
                        f.write(write_str)
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
    else:
        raise Exception("Not enough arguments")

if __name__ == "__main__":
    main()
