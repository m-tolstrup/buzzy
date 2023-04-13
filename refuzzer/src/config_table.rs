pub struct ConfigTable {
	pub seed: u32,
	pub random_instr_count: u32,
}

impl ConfigTable {
	pub fn new(_seed: u32) -> ConfigTable {
		ConfigTable {
			seed: _seed,
			random_instr_count: _seed & 111111111,
		}
	}
}
