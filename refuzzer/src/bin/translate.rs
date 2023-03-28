use rbpf::disassembler;

// cargo run --bin translate

fn main () {
	let prog = &[
		0xb7, 0x01, 0x00, 0x00, 0x28, 0x00, 0x00, 0x00, 
		0x63, 0x1a, 0xfc, 0xff, 0x00, 0x00, 0x00, 0x00, 
		0xb7, 0x01, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 
		0x63, 0x1a, 0xf8, 0xff, 0x00, 0x00, 0x00, 0x00, 
		0x61, 0xa0, 0xfc, 0xff, 0x00, 0x00, 0x00, 0x00, 
		0x61, 0xa1, 0xf8, 0xff, 0x00, 0x00, 0x00, 0x00, 
		0x0f, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 
		0x95, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 
	];

	disassembler::disassemble(prog);
}