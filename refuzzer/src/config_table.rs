use rand::prelude::*;

#[derive(Clone, Copy)]
pub struct ConfigTable {
	pub seed: u32,
	pub random_instr_count: u32,
}

impl ConfigTable {
	pub fn new(_seed: u32) -> ConfigTable {
		ConfigTable {
			seed: _seed,
			// Max instruction size is 512, i.e. 9 bits
			// Change here if you want more or fewer instructions
			random_instr_count: _seed & 0b111111111,
		}
	}

	pub fn get_rand_dst_reg(self) -> u8 {
		// TODO should not be completly random? Context matters
		let reg: u8 = match rand::thread_rng().gen_range(0..6) {
			0 => u8::from(0),
			1 => u8::from(1),
			2 => u8::from(2),
			3 => u8::from(3),
			4 => u8::from(4),
			5 => u8::from(5),
			_ => unreachable!(),
		};
		reg
	}

	pub fn get_rand_src_reg(self) -> u8 {
		// TODO should not be completly random? Context matters
		let reg: u8 = match rand::thread_rng().gen_range(0..6) {
			0 => u8::from(0),
			1 => u8::from(1),
			2 => u8::from(2),
			3 => u8::from(3),
			4 => u8::from(4),
			5 => u8::from(5),
			_ => unreachable!(),
		};
		reg
	}

	pub fn get_rand_imm(self) -> i32 {
		let imm: i32 = rand::thread_rng().gen_range(0..2147483647);
		imm
	}
}
