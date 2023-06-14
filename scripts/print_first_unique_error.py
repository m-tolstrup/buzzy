import os
import subprocess

ubpf = os.path.join("..", "ubpf", "vm")
errors = os.path.join("..", "buzzy", "logs")

error_dict = {}

for file in os.listdir(errors):
    if file.startswith("error") and file.endswith(".o"):
        error_path = os.path.join("..", "..", "buzzy", "logs", file)
        os.chdir(ubpf)
        cmd_str = "./test " + str(error_path)
        result = subprocess.run(cmd_str, shell=True)

        if result not in error_dict:
            error_dict[result] = str(file)

print(error_dict)
