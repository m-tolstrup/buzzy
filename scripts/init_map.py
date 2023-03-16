from bcc import BPF
from time import sleep

program = r"""
BPF_ARRAY(data, u64, 32);
"""

b = BPF(text=program)

while True:
    sleep(2)
    s = ""
    # Key in array is just the index, so range is 0 to 31 here
    for k, v in b["data"].items():
        if v.value != 0:
            s += f"Entry: {key.value} {value.value}\n"
    if s != "":
        print("The following entrances changed")
        print(s)
