import os

# This script looks in the logs/exp-data.txt file and prints the different errors encounted

results = {
    "valid": 0,
    "invalid": 0,
    "unmarshaling": 0,
}

def main():
    path = os.path.join("..", "buzzy", "logs", "exp-data.txt")

    lines = []

    with open(path, 'r') as file:
        lines = file.readlines()

    for line in lines:
        line = line.strip()
        if line.startswith("0"):
            results["invalid"] += 1
        elif line.startswith("1"):
            results["valid"] += 1
        elif line.startswith("unmarshaling"):
            results["unmarshaling"] += 1
            split = line.split(",")
            line = split[0][25:]
            line = line[1:] if line.startswith(" ") else line
            if line in results:
                results[line] += 1
            else:
                results[line] = 1


if __name__ == "__main__":
    main()
    print("\n")
    for result in results:
        print(f"Count: {results[result]}\tInstruction:\t{result}")
    print("\n")
