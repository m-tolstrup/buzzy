import os
import matplotlib.pyplot as plt

def main():
    path = os.path.join("..", "buzzy", "logs", "exp-data.txt")

    lines = []
    valid_program_count_list = [0] * 512
    invalid_program_count_list = [0] * 512

    with open(path, 'r') as file:
        lines = file.readlines()

    for line in lines:
        line = line.split(",")
        instr_count = int(line[-1])
        first_value = line[0]
        if first_value == "1":
            valid_program_count_list[instr_count] += 1
        else:
            invalid_program_count_list[instr_count] += 1

    percentages = []

    for i in range(512):
        if invalid_program_count_list[i] == 0:
            percentages.append(0.0)
        else:
            percentages.append((valid_program_count_list[i]/invalid_program_count_list[i])*100)


    plt.plot(percentages[1:32])
    plt.title('Random Instructions')
    plt.ylabel('Percentage of valid programs')
    plt.xlabel('Number of instructions')
    plt.show()

if __name__ == "__main__":
    main()
