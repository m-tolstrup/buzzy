pub struct ConfigTable {
	pub seed: u32,
}

impl ConfigTable {
	pub fn new(_seed: u32) -> ConfigTable {
		ConfigTable {
			seed: _seed,
		}
	}
}
