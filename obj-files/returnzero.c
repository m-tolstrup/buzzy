#include <linux/bpf.h>

// clang -target bpf -c returnzero.c -o returnzero.o

int func() {
	return 0;
}
