import os
import subprocess

prevail = os.path.join("..", "ebpf-verifier")
ubpf = os.path.join("..", "ubpf", "vm")

def main():

    error_logs = os.path.join("error-logs")

    for file in os.listdir(error_logs):
        if file.endswith(".o"):
            # Print translate script
            cmd_str = "python3 translate.py " + str(error_logs) + "/" + str(file) + " 6"
            subprocess.run(cmd_str, shell=True)

            # Print PREVAIL result
            os.chdir(prevail)
            error_path = os.path.join("..", "scripts", error_logs, file)
            cmd_str = "./check " + str(error_path)
            print("\nPREVAIL:")
            subprocess.run(cmd_str, shell=True)
            
            # Print uBPF result
            error_path = os.path.join("..", "..", "scripts", error_logs, file)
            os.chdir(ubpf)
            cmd_str = "./test " + str(error_path)
            print("\nuBPF:")
            subprocess.run(cmd_str, shell=True)
            print("\n")

if __name__ == "__main__":
    main()
