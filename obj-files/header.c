#include <linux/bpf.h>

// Header by https://scannell.io/posts/ebpf-fuzzing/

int func() {
	BPF_MOV64_IMM(BPF_REG_0, 0);
	BPF_STX_MEM(BPF_W, BPF_REG_10, BPF_REG_0, -4);
	BPF_MOV64_REG(BPF_REG_2, BPF_REG_10);
	BPF_ALU64_IMM(BPF_ADD, BPF_REG_2, -4);
	// The integer 1 is map file descriptor
	BPF_LD_MAP_FD(BPF_REG_1, 1);
	BPF_RAW_INSN(BPF_JMP | BPF_CALL, 0, 0, 0, BPF_FUNC_map_lookup_elem);
	BPF_JMP_IMM(BPF_JNE, BPF_REG_0, 0, 1);
	BPF_EXIT_INSN();
}

