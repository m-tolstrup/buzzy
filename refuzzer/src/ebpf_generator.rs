#![allow(unused_imports)]
#![allow(unreachable_code)]
#![allow(unreachable_patterns)]

use rand::prelude::*;

use rbpf::insn_builder::{
    BpfCode,
    Arch,
    Endian,
    Instruction,
    Source,
    Cond,
    MemSize,
};

use crate::symbol_table::SymbolTable;

pub struct EbpfGenerator<'a> {
    pub prog: BpfCode,
    symbol_table: SymbolTable,
    strategy: &'a str,
}

impl EbpfGenerator<'_> {
    pub fn new(_seed: u32, _strategy: &str) -> EbpfGenerator {
        EbpfGenerator { 
            prog: BpfCode::new(),
            symbol_table: SymbolTable::new(_seed),
            strategy: _strategy,
        }
    }

    pub fn generate_program(&mut self) {

        // We (almost) always init zero and push exit, so two are subtracted from the range here
        self.symbol_table.instr_count = self.symbol_table.rng.gen_range(0..511);

        match self.strategy {
            "InitZero" => {
                self.init_zero();
            },
            "Random" => {
                self.random_instructions();
            },
            "MapHeader" => {
                self.init_zero();
                self.init_map();
            },
            "RandomStackSequences" => {
                self.random_sequences();
            },
            _ => {
                //Nothing
            }
        };

        // Always push exit instruction
        self.prog.exit().push();
    }

    fn init_zero(&mut self) {
        self.prog.mov(Source::Imm, Arch::X64).set_dst(0).set_imm(0).push();
    }

    fn random_instructions(&mut self) {

        // Always initialize zero - lets more programs through the verifier
        self.prog.mov(Source::Imm, Arch::X64).set_dst(0).set_imm(0).push();

        let mut instr_gen_count: i32 = self.symbol_table.instr_count;
        
        loop {
            if instr_gen_count == 0 {
                break;
            }

            // Weighted to match number of available instructions
            match self.symbol_table.rng.gen_range(0..20) {
                0..13  => self.select_random_alu_instr(),
                13..15 => self.select_random_store_instr(),
                15..19 => self.select_random_load_instr(),
                19..20 => self.select_random_jump_instr(), // TODO weight jump based on number of different jump conditions?
                _      => unreachable!(),
            }

            instr_gen_count -= 1;
        }
    }

    pub fn random_sequences(&mut self) {
        // This generation technique is pretty stack focused right now
        let mut instr_gen_count: i32 = self.symbol_table.instr_count;
        loop {
            if instr_gen_count <= 0 {
                break;
            }

            let add_value: i32 = self.symbol_table.get_random_stack_add_value();
            let sub_value: i32 = self.symbol_table.get_random_stack_sub_value();

            let generated_count: i32 = match self.symbol_table.rng.gen_range(0..5) {
                0 => self.sequence_mov_imm_to_reg(),
                1 => self.sequence_pop_from_stack(),
                2 => self.sequence_push_to_stack(),
                3 => self.add_stack_pointer(add_value),
                4 => self.sub_stack_pointer(sub_value),
                _ => unreachable!(),
            };

            instr_gen_count -= generated_count;
        }
    }

    fn select_random_alu_instr(&mut self) {

        let dst: u8 = self.symbol_table.get_rand_dst_reg();
        let src: u8 = self.symbol_table.get_rand_src_reg();
        let imm: i32 = self.symbol_table.get_rand_imm();

        // Select the source type
        let source: Source = match self.symbol_table.rng.gen_range(0..2) {
            0 => Source::Imm,
            1 => Source::Reg,
            _ => unreachable!(),
        };

        // Choose a random (ALU) instruction and set the destination register
        // TODO swap bytes is missing
        let instruction = match self.symbol_table.rng.gen_range(0..13) {
            0  => self.prog.add(source, Arch::X64).set_dst(dst),
            1  => self.prog.sub(source, Arch::X64).set_dst(dst),
            2  => self.prog.mul(source, Arch::X64).set_dst(dst),
            3  => self.prog.div(source, Arch::X64).set_dst(dst),
            4  => self.prog.modulo(source, Arch::X64).set_dst(dst),
            5  => self.prog.bit_or(source, Arch::X64).set_dst(dst),
            6  => self.prog.bit_xor(source, Arch::X64).set_dst(dst),
            7  => self.prog.bit_and(source, Arch::X64).set_dst(dst),
            8  => self.prog.left_shift(source, Arch::X64).set_dst(dst),
            9  => self.prog.right_shift(source, Arch::X64).set_dst(dst),
            10 => self.prog.signed_right_shift(source, Arch::X64).set_dst(dst),
            11 => self.prog.mov(source, Arch::X64).set_dst(dst),
            12 => self.prog.negate(Arch::X64).set_dst(dst),
            _  => unreachable!(),
        };

        // Then, depending on the source type, set the value of the source and push it
        match source {
            Source::Imm => instruction.set_imm(imm).push(),
            Source::Reg => instruction.set_src(src).push(),
            _ => unreachable!(),
        };
    }

    pub fn select_random_store_instr(&mut self) {

        // "dst" is most likely to be stackpointer (R10) in this context?
        let dst: u8 = self.symbol_table.get_rand_dst_reg();
        let src: u8 = self.symbol_table.get_rand_src_reg();
        let imm: i32 = self.symbol_table.get_rand_imm();
        let offset: i16 = self.symbol_table.get_rand_offset();

        let mem_size: MemSize = match self.symbol_table.rng.gen_range(0..4) {
            0 => MemSize::Byte,
            1 => MemSize::HalfWord,
            2 => MemSize::Word,
            3 => MemSize::DoubleWord,
            _ => unreachable!(),
        };

        let instruction = match self.symbol_table.rng.gen_range(0..2) {
            0 => self.prog.store(mem_size).set_dst(dst).set_imm(imm).set_off(offset),
            1 => self.prog.store_x(mem_size).set_dst(dst).set_src(src).set_off(offset),
            _ => unreachable!(),
        };

        instruction.push();
    }

    pub fn select_random_load_instr(&mut self) {
        
        let dst: u8 = self.symbol_table.get_rand_dst_reg();
        // "src" is most likely to be stackpointer (R10) in this context?
        let src: u8 = self.symbol_table.get_rand_src_reg();
        let imm: i32 = self.symbol_table.get_rand_imm();
        let offset: i16 = self.symbol_table.get_rand_offset();

        let mem_size: MemSize = match self.symbol_table.rng.gen_range(0..4) {
            0 => MemSize::Byte,
            1 => MemSize::HalfWord,
            2 => MemSize::Word,
            3 => MemSize::DoubleWord,
            _ => unreachable!(),
        };

        match self.symbol_table.rng.gen_range(0..2) {
            0 => {
                match mem_size {
                    MemSize::DoubleWord => {
                        self.prog.load(mem_size).set_dst(dst).set_imm(imm).set_off(offset).push();
                        self.prog.load(mem_size).set_dst(dst).set_imm(imm).set_off(offset).push();
                    },
                    _ => { } // Only allowed for double word so do nothing
                };
            },
            1 => {self.prog.load_x(mem_size).set_dst(dst).set_src(src).set_off(offset).push();},
            // 2 => {self.prog.load_abs(mem_size).set_dst(dst).set_src(src).set_off(offset).push();}, // LEGACY
            // 3 => {self.prog.load_ind(mem_size).set_dst(dst).set_src(src).set_off(offset).push();}, // LEGACY
            _ => {unreachable!();},
        };
    }

    pub fn select_random_jump_instr(&mut self) {

        let dst: u8 = self.symbol_table.get_rand_dst_reg();
        let src: u8 = self.symbol_table.get_rand_src_reg();
        let imm: i32 = self.symbol_table.get_rand_imm();
        let offset: i16 = self.symbol_table.get_rand_offset();

        let condition: Cond = match self.symbol_table.rng.gen_range(0..11) {
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

        let source: Source = match self.symbol_table.rng.gen_range(0..2) {
            0 => Source::Imm,
            1 => Source::Reg,
            _ => unreachable!(),
        };

        // Weighted to match number of jump instructions
        let instruction = match self.symbol_table.rng.gen_range(0..12) {
            0..1  => self.prog.jump_unconditional().set_dst(dst),
            1..12 => self.prog.jump_conditional(condition, source).set_dst(dst),
            _     => unreachable!(),
        };

        match source {
            Source::Imm => instruction.set_imm(imm).set_off(offset).push(),
            Source::Reg => instruction.set_src(src).set_off(offset).push(),
            _           => unreachable!(),
        };
    }

    pub fn sequence_mov_imm_to_reg(&mut self) -> i32 {
        // Move an immediate value to a register
        // Useful for initializing - also tracks initialized registers
        let dst: u8 = self.symbol_table.get_rand_dst_reg();
        let imm: i32 = self.symbol_table.get_rand_imm();

        self.symbol_table.initialize_register(dst);

        self.prog.mov(Source::Imm, Arch::X64).set_dst(dst).set_imm(imm).push();

        return 1;
    }

    pub fn sequence_push_to_stack(&mut self) -> i32 {
        // Move the stack pointer and at store something
        let stack_pointer: u8 = 10;
        let src: u8 = self.symbol_table.get_rand_src_reg();
        let offset: i16 = 0;

        let mem_size: MemSize = match self.symbol_table.rng.gen_range(0..4) {
            0 => MemSize::Byte,
            1 => MemSize::HalfWord,
            2 => MemSize::Word,
            3 => MemSize::DoubleWord,
            _ => unreachable!(),
        };

        let move_stack_offset: i32 = match mem_size {
            MemSize::Byte       => 1,
            MemSize::HalfWord   => 2,
            MemSize::Word       => 4,
            MemSize::DoubleWord => 8,
        };

        // TODO multiple store_x after a large add to stack pointer

        let i: i32 = self.add_stack_pointer(move_stack_offset);
        self.prog.store_x(mem_size).set_dst(stack_pointer).set_src(src).set_off(offset).push();

        self.symbol_table.store_from_register(src);
        self.symbol_table.push_value_to_stack(mem_size);

        return 1 + i;
    }

    pub fn sequence_pop_from_stack(&mut self) -> i32 {
        let stack_pointer: u8 = 10;
        let dst: u8 = self.symbol_table.get_rand_dst_reg();
        let offset: i16 = 0;

        let mem_size: MemSize = match self.symbol_table.rng.gen_range(0..4) {
            0 => MemSize::Byte,
            1 => MemSize::HalfWord,
            2 => MemSize::Word,
            3 => MemSize::DoubleWord,
            _ => unreachable!(),
        };

        let move_stack_offset: i32 = match mem_size {
            MemSize::Byte       => 1,
            MemSize::HalfWord   => 2,
            MemSize::Word       => 4,
            MemSize::DoubleWord => 8,
        };

        // TODO multiple load_x then a large sub from stack pointer

        self.prog.load_x(mem_size).set_dst(dst).set_src(stack_pointer).set_off(offset).push();
        let i: i32 = self.sub_stack_pointer(move_stack_offset);

        self.symbol_table.load_to_register(dst);
        self.symbol_table.pop_value_from_stack(mem_size);

        return 1 + i;
    }

    pub fn add_stack_pointer(&mut self, move_stack_offset: i32) -> i32 {
        self.symbol_table.add_stack_pointer(move_stack_offset);
        self.prog.add(Source::Imm, Arch::X64).set_dst(10).set_imm(move_stack_offset).push();
        return 1;
    }

    pub fn sub_stack_pointer(&mut self, move_stack_offset: i32) -> i32 {
        self.symbol_table.sub_stack_pointer(move_stack_offset);
        self.prog.sub(Source::Imm, Arch::X64).set_dst(10).set_imm(move_stack_offset).push();
        return 1;
    }
    
    pub fn init_map(&mut self) {
        // Prepare the stack for "map_lookup_elem"
        //self.prog.mov(Source::Imm, Arch::X64).set_dst(0).set_imm(0).push();

        self.prog.store_x(MemSize::Word).set_dst(10).set_src(0).set_off(-4).push();

        self.prog.mov(Source::Reg, Arch::X64).set_dst(2).set_src(10).push();
        self.prog.add(Source::Imm, Arch::X64).set_dst(2).set_imm(-4).push();

        // Make the call to "map_lookup_elem"
        //BPF_LD_MAP_FD(BPF_REG_1, BPF_TRIAGE_MAP_FD)
        self.prog.mov(Source::Imm, Arch::X64).set_dst(1).set_imm(1).push();

        // integer value in 'imm' field of BPF_CALL instruction selects which helper function eBPF program intends to call
        // https://github.com/torvalds/linux/blob/master/include/uapi/linux/bpf.h#L5506
        self.prog.call().set_dst(0).set_src(1).set_off(0).set_imm(0x00_00_00_01).push();
        
        // Verify the map so that we can use it
        self.prog.jump_conditional(Cond::NotEquals, Source::Imm).set_dst(0).set_imm(0).set_off(1).push();
        self.prog.exit().push();

        // Initialize two registers by reading from map?
        // BPF_LDX_MEM(BPF_DW, this->reg1, BPF_REG_0, 0) this->reg1?
        self.prog.load_x(MemSize::DoubleWord).set_dst(1).set_src(0).set_off(0).push();
        self.prog.load_x(MemSize::DoubleWord).set_dst(2).set_src(0).set_off(8).push();

    }
}
