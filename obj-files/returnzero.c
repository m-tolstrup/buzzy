#include <linux/bpf.h>

int func() {
	BPF_MOV64_IMM(BPF_REG_0, 3);
	BPF_MOV64_IMM(BPF_REG_1, 0);
	BPF_MOV64_REG(BPF_REG_1, BPF_REG_0);
	return 0;
}
