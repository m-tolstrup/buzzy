use rand::prelude::*;

use rbpf::insn_builder::{
    MemSize,
};

pub struct ConfigTable {
	seed: u32,
	
	pub instr_count: u32,
	select_edge_cases: bool,
	stored_registers: Vec<u8>,
	loaded_registers: Vec<u8>,
	stack_pointer_position: u16,
	stack_total_size_used: u16,	
}

impl ConfigTable {
	pub fn new(_seed: u32, _random_choices: u8) -> ConfigTable {
		ConfigTable {
			// ***** VARIABLES MANUALLY SET FOR EXPERIMENTS - AFFECTING RANDOM CHOICES, ETC ***** //
			select_edge_cases: true, // select edge case values
			// select_edge_cases: _random_choices & (1 << 0) != 0,

			// ***** VARIABLES FOR RANDOM CHOICES ***** //
			seed: _seed,
			// Max instruction size is 512, i.e. 9 bits
			// Change here if you want more or fewer instructions
			instr_count: _seed & 0b111111111,

			// ***** VARIABLES TO TRACK PROGRAM ***** //
			stored_registers: Vec::new(), // registers where a store has been performed, but no following load
			loaded_registers: Vec::new(), // registers where a load has been performed, but no following store
			stack_total_size_used: 0, // how many bytes have been pushed to the stack (popped are subtracted)
			stack_pointer_position: 0, // stack position indicated in bytes
		}
	}

	pub fn get_rand_dst_reg(&mut self) -> u8 {
		// If something has been stored from the register, it is probably a good dst for a new value
		let reg: u8;
		if self.stored_registers.is_empty() {
			reg = rand::thread_rng().gen_range(0..6);
		} else {
			reg = match self.stored_registers.pop() {
				Some(num) => num,
				None => rand::thread_rng().gen_range(0..6),
			};
		}
		reg
	}

	pub fn get_rand_src_reg(&mut self) -> u8 {
		// If something has been loaded into a register, it is probably a good src
		let reg: u8;
		if self.loaded_registers.is_empty() {
			reg = rand::thread_rng().gen_range(0..6);
		} else {
			reg = match self.loaded_registers.pop() {
				Some(num) => num,
				None => rand::thread_rng().gen_range(0..6),
			};
		}
		reg
	}

	pub fn store_from_register(&mut self, register: u8) {
		// Store if a registers contents have been pushed to the stack
		if !self.stored_registers.contains(&register) {
			self.stored_registers.push(register);
		}
	}

	pub fn load_from_register(&mut self, register: u8) {
		// Store if a register has been used to store value popped from stack
		if !self.loaded_registers.contains(&register) {
			self.loaded_registers.push(register);
		}
	}

	pub fn get_rand_imm(self) -> i32 {
		// Return a random immediate
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
		// Return a random offset
		let offset: i16;
		if self.select_edge_cases {
			offset = match rand::thread_rng().gen_range(0..6) {
				0 => 0,
				1 => 1, // Byte
				2 => 2, // Half Word
				3 => 4, // Word
				4 => 8,	// Double Word
				5 => 32767,
				_ => unreachable!()
			};
		} else {
			offset = rand::thread_rng().gen_range(0..32767);
		}
		offset
	}

	pub fn stack_to_top(self) -> u16 {
		// Return number of bytes needed to set stack pointer at the top of the stack
		512 - self.stack_pointer_position
	}

	pub fn stack_to_bottom(self) -> u16 {
		// Return number of bytes needed to set stack pointer at the bottom of the stack
		self.stack_pointer_position
	}

	pub fn move_stack_pointer(&mut self, number: u16) {
		// Check if the stack pointer can be moved the chosen number of bytes - then move it
		if number > 0 && self.stack_pointer_position + number <= 512 {
				self.stack_pointer_position += number;
		} else if self.stack_pointer_position - number >= 0 {
			self.stack_pointer_position -= number;
		}
	}

	pub fn push_value_to_stack(&mut self, mem_size: MemSize) {
		// Find number of bytes pushed to the stack and track it
		let bytes: u16 = match mem_size {
			MemSize::Byte       => 1,
			MemSize::HalfWord   => 2,
			MemSize::Word       => 4,
			MemSize::DoubleWord => 8,
		};

		self.stack_total_size_used += bytes;
	}

	pub fn pop_value_from_stack(&mut self, mem_size: MemSize) {
		// Find number of bytes popped from the stack and track it
		let bytes: u16 = match mem_size {
			MemSize::Byte       => 1,
			MemSize::HalfWord   => 2,
			MemSize::Word       => 4,
			MemSize::DoubleWord => 8,
		};

		self.stack_total_size_used -= bytes;
	}
}
