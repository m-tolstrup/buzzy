import os

results = {
    "total_count": 0,
    "avg_time": 0.0,
    "avg_mem": 0.0,
    "valid": 0,
    "v_min_time": 60.0,
    "v_max_time": 0.0,
    "v_avg_time": 0.0,
    "v_min_mem": 10_000,
    "v_max_mem": 0,
    "v_avg_mem": 0.0,
    "invalid": 0,
    "i_min_time": 60.0,
    "i_max_time": 0.0,
    "i_avg_time": 0.0,
    "i_min_mem": 10_000,
    "i_max_mem": 0,
    "i_avg_mem": 0.0,
    "jump_err": 0,
    "lddw_err": 0
}

def handle_line(line):
    
    split = line.split(",")
    status, time, memory = int(split[0]), float(split[1]), int(split[2])

    total = results["valid"] + results["invalid"]
    results["avg_time"] = (results["avg_time"] * total + time) / (total + 1)
    results["avg_mem"] = (results["avg_mem"] * total + memory) / (total + 1)

    if status:
        if time > results["v_max_time"]:
            results["v_max_time"] = time
        if time < results["v_min_time"]:
            results["v_min_time"] = time
        results["v_avg_time"] = (results["v_avg_time"] * results["valid"] + time) / (results["valid"] + 1)

        if memory > results["v_max_mem"]:
            results["v_max_mem"] = memory
        if memory < results["v_min_mem"]:
            results["v_min_mem"] = memory
        results["v_avg_mem"] = (results["v_avg_mem"] * results["valid"] + memory) / (results["valid"] + 1)
    else:
        if time > results["i_max_time"]:
            results["i_max_time"] = time
        if time < results["i_min_time"]:
            results["i_min_time"] = time
        results["i_avg_time"] = (results["i_avg_time"] * results["invalid"] + time) / (results["invalid"] + 1)

        if memory > results["i_max_mem"]:
            results["i_max_mem"] = memory
        if memory < results["i_min_mem"]:
            results["i_min_mem"] = memory
        results["i_avg_mem"] = (results["i_avg_mem"] * results["invalid"] + memory) / (results["invalid"] + 1)

def main():
    path = os.path.join("..", "buzzy", "logs", "exp-data.txt")

    lines = []

    with open(path, 'r') as file:
        lines = file.readlines()

    for line in lines:
        line = line.strip()
        if line == "":
            continue
        elif "jump to middle of lddw" in line:
            results["lddw_err"] += 1
        elif "jump out of bounds" in line:
            results["jump_err"] += 1
        elif line.startswith("0"):
            results["invalid"] += 1
            handle_line(line)
        elif line.startswith("1"):
            results["valid"] += 1
            handle_line(line)

        results["total_count"] += 1

def print_results():
    min_time = min(results['v_min_time'], results['i_min_time'])
    max_time = max(results['v_max_time'], results['i_max_time'])
    min_mem = min(results['v_min_mem'], results['i_min_mem'])
    max_mem = max(results['v_max_mem'], results['i_max_mem'])

    result_string = f""
    result_string += f"\n\t\t***** RESULTS *****\n\n"
    result_string += f"TOTAL LINES PROCESSED: {results['total_count']} ({results['valid']+results['invalid']})\n"
    result_string += f"\tTIME\t MIN: {min_time}\t MAX: {max_time}\t AVG: {results['avg_time']}\n"
    result_string += f"\tMEMORY\t MIN: {min_mem}\t MAX: {max_mem}\t AVG: {results['avg_mem']}\n"
    result_string += f"\n\tJUMP OUT OF BOUNDS ERRORS: {results['jump_err']}\n"
    result_string += f"\tJUMP OUT TO LDDW ERRORS: {results['lddw_err']}\n\n"
    result_string += f"VALID PROGRAMS: {results['valid']}\n"
    result_string += f"\tTIME\t MIN: {results['v_min_time']}\t MAX: {results['v_max_time']}\t AVG: {results['v_avg_time']}\n"
    result_string += f"\tMEMORY\t MIN: {results['v_min_mem']}\t MAX: {results['v_max_mem']}\t AVG: {results['v_avg_mem']}\n\n"
    result_string += f"INVALID PROGRAMS: {results['invalid']}\n"
    result_string += f"\tTIME\t MIN: {results['i_min_time']}\t MAX: {results['i_max_time']}\t AVG: {results['i_avg_time']}\n"
    result_string += f"\tMEMORY\t MIN: {results['i_min_mem']}\t MAX: {results['i_max_mem']}\t AVG: {results['i_avg_mem']}\n\n"
    print(result_string)

if __name__ == "__main__":
    main()
    print_results()
