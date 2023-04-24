use rand::prelude::*;

#[derive(Clone, Copy)]
pub struct ConfigTable {
	select_edge_cases: bool,
	pub seed: u32,
	pub random_instr_count: u32,
}

impl ConfigTable {
	pub fn new(_seed: u32, _random_choices: u8) -> ConfigTable {
		ConfigTable {
			// ***** VARIABLES MANUALLY SET FOR EXPERIMENTS ***** //
			select_edge_cases: true,
			// select_edge_cases: _random_choices & (1 << 0) != 0,

			// ***** VARIABLES FOR RANDOM CHOICES ***** //
			seed: _seed,
			// Max instruction size is 512, i.e. 9 bits
			// Change here if you want more or fewer instructions
			random_instr_count: _seed & 0b111111111,
		}
	}

	pub fn get_rand_dst_reg(self) -> u8 {
		// TODO should not be completly random? Context matters
		let reg: u8 = rand::thread_rng().gen_range(0..6);
		reg
	}

	pub fn get_rand_src_reg(self) -> u8 {
		// TODO should not be completly random? Context matters
		let reg: u8 = rand::thread_rng().gen_range(0..6);
		reg
	}

	pub fn get_rand_imm(self) -> i32 {
		let imm: i32;
		if self.select_edge_cases {
			imm = match rand::thread_rng().gen_range(0..2) {
				0 => 0,
				1 => 2147483647,
				_ => unreachable!()
			};
		} else {
			imm = rand::thread_rng().gen_range(0..2147483647);
		}
		imm
	}
	
	pub fn get_rand_offset(self) -> i16 {
		let offset: i16;
		if self.select_edge_cases {
			offset = match rand::thread_rng().gen_range(0..2) {
				0 => 0,
				1 => 32767,
				_ => unreachable!()
			};
		} else {
			offset = rand::thread_rng().gen_range(0..32767);
		}
		offset
	}
}
