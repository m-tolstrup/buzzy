import os
import subprocess

ubpf = os.path.join("..", "ubpf", "vm")
errors = os.path.join("..", "..", "buzzy", "logs")

# No stderr in uBPF so errors cant be counted
# error_dict = {}

def main():
    os.chdir(ubpf)
    for file in os.listdir(errors):
        if file.startswith("error") and file.endswith(".o"):
            error_path = os.path.join(errors, file)
            cmd_str = "./test -j " + str(error_path)
            result = subprocess.run(cmd_str, shell=True)
            print(str(file)+"\n")
            # if result not in error_dict:
                # error_dict[result] = str(file)

if __name__ == "__main__":
    # Indirectly prints all found errors by running erroneous programs in uBPF
    # uBPF stderr is not the reported error when running
    # Maybe create issue on their repo to extend this
    main()
