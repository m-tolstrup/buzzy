use rand::prelude::*;

use rbpf::insn_builder::{
    MemSize,
	Source,
	Cond,
};


pub struct SymbolTable {
	pub rng: ThreadRng,
	seed: u32,
	
	pub const_instr_count: i32,
	instr_count: i32,
	max_alu: i32,
	max_jump: i32,
	select_numeric_edge_cases: bool,
	select_random_registers: bool,
	initialized_registers: Vec<u8>,
	stored_registers: Vec<u8>,
	loaded_registers: Vec<u8>,
	stack_height: i32,
}

impl SymbolTable {
	pub fn new(_seed: u32) -> SymbolTable {
		SymbolTable {
			rng: rand::thread_rng(),
			// ***** VARIABLES FOR RANDOM CHOICES BASED ON SEED ***** //
			seed: _seed,

			// Set when gen_instr_count is called
			instr_count: 0,
			const_instr_count: 0,


			// ***** VARIABLES MANUALLY SET FOR EXPERIMENTS - AFFECTING RANDOM CHOICES, ETC ***** //

			// Maximum number of ALU instructions in a row for sequences
			max_alu: 5,
			// Maximum number of JUMP instructions in a row for sequences
			max_jump: 1,
			// Select edge case values
			select_numeric_edge_cases: true,
			// select_edge_cases: _random_choices & (1 << 0) != 0,

			// Select completely random registers
			// When false, only register 0 to 5 is selected
			select_random_registers: false,
			// select_random_registers: _random_choices & (2 << 0) != 0,

			// ***** VARIABLES TO TRACK PROGRAM ***** //

			// Initialized registers
			initialized_registers: Vec::new(),
			// Registers where a store has been performed, but no following load
			stored_registers: Vec::new(),
			// Registers where a load has been performed, but no following store
			loaded_registers: Vec::new(),
 			// How many bytes have been pushed to the stack (popped are subtracted)
			// Used to give a rough idea of stack use - not intended to be accurate
			stack_height: 0,
		}
	}

	pub fn get_instr_count(&mut self) -> i32 {
		self.instr_count
	}

	pub fn set_instr_count(&mut self, i:i32) {
		self.instr_count = i;
	}

	pub fn gen_instr_count(&mut self) -> i32 {
		// One in ten programs are 32 instructions or less
		let instr_count = match self.rng.gen_range(0..100) {
			0..90   => self.rng.gen_range(1..33),
			90..100 => self.rng.gen_range(33..511),
			_       => unreachable!(),
		};
		self.const_instr_count = instr_count;
		instr_count
	}

	pub fn get_max_alu_instr(&mut self) -> i32 {
		self.max_alu
	}

	pub fn get_max_jump_instr(&mut self) -> i32 {
		self.max_jump
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

	pub fn get_init_register(&mut self, index: usize) -> u8 {
		let reg: u8 = self.initialized_registers[index];
		reg
	}

	pub fn initialize_register(&mut self, register: u8) {
		// Keep track of initialized registers
		if !self.initialized_registers.contains(&register) {
			self.initialized_registers.push(register);
		}
	}

	pub fn initialized_register_count(&mut self) -> usize {
		let i: usize = self.initialized_registers.len();
		i
	}

	pub fn store_from_register(&mut self, register: u8) {
		// Store if a registers contents have been pushed to the stack
		if !self.stored_registers.contains(&register) {
			self.stored_registers.push(register);
		}
	}

	pub fn load_to_register(&mut self, register: u8) {
		// Store if a register has been used to store value popped from stack
		if !self.loaded_registers.contains(&register) {
			self.loaded_registers.push(register);
		}
	}

	pub fn get_rand_imm(&mut self) -> i32 {
		// Return a random immediate
		let imm: i32;
		if self.select_numeric_edge_cases {
			imm = match self.rng.gen_range(0..6) {
				0 => 0,
				1 => 1,
				2 => -1,
				3 => -2147483648,
				4 => 2147483647,
				5 => self.rng.gen_range(-2147483648..=2147483647),
				_ => unreachable!(),
			};
		} else {
			imm = self.rng.gen_range(-2147483648..=2147483647);
		}
		imm
	}
	
	pub fn get_rand_offset(&mut self) -> i16 {
		// Return a random offset
		let offset: i16;
		if self.select_numeric_edge_cases {
			offset = match self.rng.gen_range(0..9) {
				0 => 0,
				1 => -1,
				2 => 1, // Byte
				3 => 2, // Half Word
				4 => 4, // Word
				5 => 8,	// Double Word
				6 => 32767,
				7 => -32768,
				8 => self.rng.gen_range(-32768..=32767),
				_ => unreachable!(),
			};
		} else {
			offset = self.rng.gen_range(-32768..=32767);
		}
		offset
	}

	pub fn get_rand_mem_size(&mut self) -> MemSize {
		let mem_size: MemSize = match self.rng.gen_range(0..4) {
            0 => MemSize::Byte,
            1 => MemSize::HalfWord,
            2 => MemSize::Word,
            3 => MemSize::DoubleWord,
            _ => unreachable!(),
        };
		mem_size
	}

	pub fn get_mem_size_offset(&self, mem_size: MemSize) -> i32 {
		let bytes: i32 = match mem_size {
			MemSize::Byte 	    => 1,
			MemSize::HalfWord   => 2,
			MemSize::Word       => 4,
			MemSize::DoubleWord => 8,
		};
		bytes
	}

	pub fn get_rand_jump_condition(&mut self) -> Cond {
		let condition: Cond = match self.rng.gen_range(0..11) {
            0  => Cond::BitAnd,
            1  => Cond::Equals,
            2  => Cond::Greater,
            3  => Cond::GreaterEquals,
            4  => Cond::GreaterEqualsSigned,
            5  => Cond::GreaterSigned,
            6  => Cond::Lower,
            7  => Cond::LowerEquals,
            8  => Cond::LowerEqualsSigned,
            9  => Cond::LowerSigned,
            10 => Cond::NotEquals,
            _  => unreachable!(),
        };
		condition
	}

	pub fn get_rand_source(&mut self) -> Source {
		let source: Source = match self.rng.gen_range(0..2) {
            0 => Source::Imm,
            1 => Source::Reg,
            _ => unreachable!(),
        };
		source
	}

	pub fn stack_to_top(&self) -> i32 {
		// Return number of bytes to the stack top
		512 - self.stack_height
	}

	pub fn stack_to_bottom(&self) -> i32 {
		// Return number of bytes to the stack bottom
		self.stack_height
	}

	pub fn get_random_stack_add_value(&mut self) -> i32 {
		let value: i32;
		if self.select_numeric_edge_cases {
			value = match self.rng.gen_range(0..6) {
				0 => -1,
				1 => 1,
				2 => 2,
				3 => 4,
				4 => 8,
				5 => self.stack_to_top(),
				_ => unreachable!()
			};
		} else {
			value = self.rng.gen_range(1..self.stack_to_top()+1)
		}
		value
	}

	pub fn get_random_stack_sub_value(&mut self) -> i32 {
		let value: i32;
		if self.select_numeric_edge_cases {
			value = match self.rng.gen_range(0..6) {
				0 => -1,
				1 => 1,
				2 => 2,
				3 => 4,
				4 => 8,
				5 => self.stack_to_bottom(),
				_ => unreachable!(),
			};
		} else {
			value = self.rng.gen_range(1..self.stack_to_bottom()+1)
		}
		value
	}

	pub fn push_to_stack(&mut self, mem_size: MemSize) {
		// Track how many bytes are stored on the stack
		let bytes: i32 = self.get_mem_size_offset(mem_size);

		// Keep the stored memory at a maximum of 512
		if self.stack_height + bytes > 512 {
			self.stack_height = 512;
		} else {
			self.stack_height += bytes;
		}	
	}

	pub fn pop_from_stack(&mut self, mem_size: MemSize) {
		// Track how many bytes are stored on the stack
		let bytes: i32 = self.get_mem_size_offset(mem_size);

		// Keep the stored memory at a maximum of 512
		if self.stack_height - bytes < 0 {
			self.stack_height = 0;
		} else {
			self.stack_height -= bytes;
		}
	}
}
