from bcc import BPF
from time import sleep

program = r"""
BPF_ARRAY(data, u64, 32);

int test_map(void *ctx) {
    u64 uid;
    u64 counter = 0;
    u64 *p;

    uid = bpf_get_current_uid_gid() & 0xFFFFFFFF;
    p = data.lookup(&counter);
    
    if (p != 0) {
        counter = *p;
    }

    counter++;
    data.update(&counter, &uid);
    return 0;
}
"""

b = BPF(text=program)
syscall = b.get_syscall_fnname("execve")
b.attach_kprobe(event=syscall, fn_name="test_map")

while True:
    sleep(2)
    s = ""
    # Key in array is just the index, so range is 0 to 31 here
    for k, v in b["data"].items():
        if v.value != 0:
            s += f"Entry: {k.value} {v.value}\n"
    if s != "":
        print("The following entrances changed")
        print(s)
