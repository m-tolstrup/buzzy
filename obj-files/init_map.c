#include <linux/bpf.h>

int func () {
    int map_fd = bpf_map_create(4, 0x150, 1);
    BPF_LD_MAP_FD(BPF_REG_1, map_fd);
    BPF_MOV64_IMM(BPF_REG_0, 0);
    BPF_EXIT_INSN();
}
