import os
import sys
import subprocess

prevail = os.path.join("..", "ebpf-verifier")
ubpf = os.path.join("..", "ubpf", "vm")
logs = os.path.join("..", "buzzy", "logs")

def main():

    if len(sys.argv) > 2:
        error_file_num = sys.argv[1]
        instr_count = sys.argv[2]

        file_ending = error_file_num + ".o"

        for file in os.listdir(logs):
            if file.endswith(file_ending):
                # Print translate script
                cmd_str = "python3 translate.py " + error_file_num + " " + instr_count
                subprocess.run(cmd_str, shell=True)

                # Print PREVAIL result
                os.chdir(prevail)
                error_path = os.path.join(logs, file)
                cmd_str = "./check " + str(error_path)
                print("\nPREVAIL:")
                subprocess.run(cmd_str, shell=True)
                    
                # Print uBPF result
                error_path = os.path.join("..", logs, file)
                os.chdir(ubpf)
                cmd_str = "./test " + str(error_path)
                print("\nuBPF:")
                subprocess.run(cmd_str, shell=True)
                print("\n")
    else:
        raise Exception("Not enough arguments")


if __name__ == "__main__":
    main()
