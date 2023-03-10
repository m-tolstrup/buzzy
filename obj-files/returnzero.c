#include <linux/bpf.h>

// Compile with: clang -target bpf -c returnzero.o -o returnzero.o

int func() {
	return 0;
}
