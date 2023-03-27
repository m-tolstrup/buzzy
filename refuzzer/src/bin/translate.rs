use rbpf::disassembler;

// cargo run --bin translate

fn main () {
	let prog = &[
		0xb7, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 
		0x7b, 0x2a, 0xf0, 0xff, 0x00, 0x00, 0x00, 0x00, 
		0xbf, 0x21, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 
		0x85, 0x10, 0x00, 0x00, 0xff, 0xff, 0xff, 0xff, 
		0x79, 0xa3, 0xf0, 0xff, 0x00, 0x00, 0x00, 0x00, 
		0xb7, 0x02, 0x00, 0x00, 0x0a, 0x00, 0x00, 0x00, 
		0x7b, 0x2a, 0xd0, 0xff, 0x00, 0x00, 0x00, 0x00, 
		0x18, 0x04, 0x00, 0x00, 0xfc, 0xff, 0xff, 0xff, 
		0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 
		0x7b, 0x4a, 0xd8, 0xff, 0x00, 0x00, 0x00, 0x00, 
		0xbf, 0x31, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 
		0x85, 0x10, 0x00, 0x00, 0xff, 0xff, 0xff, 0xff, 
		0x79, 0xa2, 0xd0, 0xff, 0x00, 0x00, 0x00, 0x00, 
		0xb7, 0x01, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 
		0x7b, 0x1a, 0xe0, 0xff, 0x00, 0x00, 0x00, 0x00, 
		0x85, 0x10, 0x00, 0x00, 0xff, 0xff, 0xff, 0xff, 
		0x79, 0xa3, 0xd8, 0xff, 0x00, 0x00, 0x00, 0x00, 
		0x79, 0xa2, 0xe0, 0xff, 0x00, 0x00, 0x00, 0x00, 
		0x79, 0xa1, 0xf0, 0xff, 0x00, 0x00, 0x00, 0x00, 
		0x85, 0x10, 0x00, 0x00, 0xff, 0xff, 0xff, 0xff, 
		0xb7, 0x02, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 
		0x7b, 0x2a, 0xe8, 0xff, 0x00, 0x00, 0x00, 0x00, 
		0xbf, 0x21, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 
		0x85, 0x10, 0x00, 0x00, 0xff, 0xff, 0xff, 0xff, 
		0x79, 0xa5, 0xe8, 0xff, 0x00, 0x00, 0x00, 0x00, 
		0x79, 0xa4, 0xf0, 0xff, 0x00, 0x00, 0x00, 0x00, 
		0xb7, 0x01, 0x00, 0x00, 0x85, 0x00, 0x00, 0x00, 
		0xbf, 0x42, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 
		0xbf, 0x43, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 
		0x85, 0x10, 0x00, 0x00, 0xff, 0xff, 0xff, 0xff, 
		0x79, 0xa4, 0xe8, 0xff, 0x00, 0x00, 0x00, 0x00, 
		0x79, 0xa3, 0xf0, 0xff, 0x00, 0x00, 0x00, 0x00, 
		0xb7, 0x01, 0x00, 0x00, 0x50, 0x00, 0x00, 0x00, 
		0xbf, 0x32, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 
		0x85, 0x10, 0x00, 0x00, 0xff, 0xff, 0xff, 0xff, 
		0x85, 0x10, 0x00, 0x00, 0xff, 0xff, 0xff, 0xff, 
		0x61, 0xa0, 0xfc, 0xff, 0x00, 0x00, 0x00, 0x00, 
		0x95, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 
	];

	disassembler::disassemble(prog);
}
