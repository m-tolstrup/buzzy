use rand::prelude::*;

use rbpf::insn_builder::{
    MemSize,
};


pub struct SymbolTable {
	pub rng: ThreadRng,
	seed: u32,
	
	pub instr_count: u32,
	select_numeric_edge_cases: bool,
	select_random_registers: bool,
	stored_registers: Vec<u8>,
	loaded_registers: Vec<u8>,
	stack_pointer_position: u16,
	stack_total_size_used: u16,	
}

impl SymbolTable {
	pub fn new(_seed: u32) -> SymbolTable {
		SymbolTable {
			rng: rand::thread_rng(),
			// ***** VARIABLES FOR RANDOM CHOICES BASED ON SEED ***** //
			seed: _seed,
			// Max instruction size is 512, i.e. 9 bits
			// Change here if you want more or fewer instructions
			instr_count: _seed & 0b111111111,

			// ***** VARIABLES MANUALLY SET FOR EXPERIMENTS - AFFECTING RANDOM CHOICES, ETC ***** //

			// Select edge case values
			select_numeric_edge_cases: true,
			// select_edge_cases: _random_choices & (1 << 0) != 0,

			// Select completely random registers
			// When false, only register 0 to 5 is selected
			select_random_registers: false,
			// select_random_registers: _random_choices & (2 << 0) != 0,

			// ***** VARIABLES TO TRACK PROGRAM ***** //

			// Registers where a store has been performed, but no following load
			stored_registers: Vec::new(),
			// Registers where a load has been performed, but no following store
			loaded_registers: Vec::new(),
 			// How many bytes have been pushed to the stack (popped are subtracted)
			// Used to give a rough idea of stack use - not intended to be accurate
			stack_total_size_used: 0,
			// Stack pointer position indicated in bytes
			stack_pointer_position: 0,
		}
	}

	// ***** Symbol table functions are purposefully not 100% accurate ***** //
	// ***** The reason being that the fuzzer should generate some strange programs that are not too correct ***** //

	pub fn get_rand_dst_reg(&mut self) -> u8 {
		// If something has been stored from the register, it is probably a good dst for a new value
		let reg: u8;
		if self.stored_registers.is_empty() {
			if self.select_random_registers {
				reg = self.rng.gen_range(0..11);
			} else {
				reg = self.rng.gen_range(0..6);
			}
		} else {
			reg = match self.stored_registers.pop() {
				Some(num) => num,
				None => self.rng.gen_range(0..6),
			};
		}
		reg
	}

	pub fn get_rand_src_reg(&mut self) -> u8 {
		// If something has been loaded into a register, it is probably a good src
		let reg: u8;
		if self.loaded_registers.is_empty() {
			if self.select_random_registers {
				reg = self.rng.gen_range(0..11);
			} else {
				reg = self.rng.gen_range(0..6);
			}
		} else {
			reg = match self.loaded_registers.pop() {
				Some(num) => num,
				None => self.rng.gen_range(0..6),
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

	pub fn get_rand_imm(&mut self) -> i32 {
		// Return a random immediate
		let imm: i32;
		if self.select_numeric_edge_cases {
			imm = match self.rng.gen_range(0..2) {
				0 => 0, // TODO something inbetween the two? A branch generating a random number?
				1 => 2147483647,
				_ => unreachable!()
			};
		} else {
			imm = self.rng.gen_range(0..2147483647);
		}
		imm
	}
	
	pub fn get_rand_offset(&mut self) -> i16 {
		// Return a random offset
		let offset: i16;
		if self.select_numeric_edge_cases {
			offset = match self.rng.gen_range(0..6) {
				0 => 0,
				1 => 1, // Byte
				2 => 2, // Half Word
				3 => 4, // Word
				4 => 8,	// Double Word
				5 => 32767,
				_ => unreachable!()
			};
		} else {
			offset = self.rng.gen_range(0..32767);
		}
		offset
	}

	pub fn stack_to_top(&self) -> u16 {
		// Return number of bytes needed to set stack pointer at the top of the stack
		512 - self.stack_pointer_position
	}

	pub fn stack_to_bottom(&self) -> u16 {
		// Return number of bytes needed to set stack pointer at the bottom of the stack
		self.stack_pointer_position
	}

	pub fn move_stack_pointer(&mut self, number: u16) {
		// Check if the stack pointer can be moved the chosen number of bytes - then move it
		if number > 0 && self.stack_pointer_position + number <= 512 {
				self.stack_pointer_position += number;
		} else {
			self.stack_pointer_position -= number;
		}
	}

	pub fn push_value_to_stack(&mut self, mem_size: MemSize) {
		let bytes: u16 = match mem_size {
			MemSize::Byte 	    => 1,
			MemSize::HalfWord   => 2,
			MemSize::Word       => 4,
			MemSize::DoubleWord => 8,
		};

		// Keep the stored memory at a maximum of 512
		if self.stack_total_size_used + bytes > 512 {
			self.stack_total_size_used = 512;
		} else {
			self.stack_total_size_used += bytes;
		}	
	}

	pub fn pop_value_from_stack(&mut self, mem_size: MemSize) {
		let bytes: u16 = match mem_size {
			MemSize::Byte 	    => 1,
			MemSize::HalfWord   => 2,
			MemSize::Word       => 4,
			MemSize::DoubleWord => 8,
		};

		self.stack_total_size_used -= bytes;
	}
}