# Code written by Liz Rice at:
# https://github.com/lizrice/learning-ebpf/blob/main/chapter2/hello.py

# Run this to test a simple Hello World program written using BCC

from bcc import BPF

program = r"""
int hello(void *ctx) {
    bpf_trace_printk("Hello World!");
    return 0;
}
"""

b = BPF(text=program)
syscall = b.get_syscall_fnname("execve")
b.attach_kprobe(event=syscall, fn_name="hello")

b.trace_print()
