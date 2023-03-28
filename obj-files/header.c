#include <linux/bpf.h>

// Header by https://scannell.io/posts/ebpf-fuzzing/

int func() {
	BPF_MOV64_IMM(BPF_REG_0, 0);
	BPF_STX_MEM(BPF_W, BPF_REG_10, BPF_REG_0, -4);
	BPF_MOV64_REG(BPF_REG_2, BPF_REG_10);
	BPF_ALU64_IMM(BPF_ADD, BPF_REG_2, -4);
	// The integer 1 is map file descriptor
	BPF_EXIT_INSN();
}

