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
		let reg: u8 = 0;
		reg
	}

	pub fn get_rand_src_reg(self) -> u8 {
		let reg: u8 = 0;
		reg
	}

	pub fn get_rand_imm(self) -> i32 {
		let imm: i32 = 42;
		imm
	}
}
