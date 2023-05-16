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
    pub symbol_table: SymbolTable,
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
        let instr_count = self.symbol_table.gen_instr_count();
        self.symbol_table.set_instr_count(instr_count);

        match self.strategy {
            "InitZero" => {
                self.init_zero();
            },
            "Random" => {
                self.init_zero();
                self.random_instructions();
            },
            "MapHeader" => {
                self.init_zero();
                self.init_map();
                self.footer();
            },
            "RandomStackSequences" => {
                self.init_zero();
                self.random_stack_sequences();
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

        let mut instr_gen_count: i32 = self.symbol_table.get_instr_count();
        
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

    fn random_stack_sequences(&mut self) {
        // This generation technique is pretty stack focused right now
        let mut instr_gen_count: i32 = self.symbol_table.get_instr_count();
        loop {
            if instr_gen_count <= 0 {
                break;
            }

            let generated_count: i32 = match self.symbol_table.rng.gen_range(0..5) {
                0 => self.sequence_mov_imm_to_reg(),
                1 => self.sequence_pop_from_stack(),
                2 => self.sequence_push_to_stack(),
                3 => self.random_alu_wrapper(),
                4 => self.random_jump_wrapper(),
                _ => unreachable!(),
            };

            instr_gen_count -= generated_count;
        }
    }

    fn random_alu_wrapper(&mut self) -> i32{
        let max_alu: i32 = self.symbol_table.get_max_alu_instr();
        let instr_gen_count: i32 = self.symbol_table.rng.gen_range(1..max_alu+1);
        
        for _ in 1..instr_gen_count {
            self.select_random_alu_instr();
        }

        return instr_gen_count;
    }

    fn random_jump_wrapper(&mut self) -> i32{
        let max_jump: i32 = self.symbol_table.get_max_jump_instr();
        let instr_gen_count: i32 = self.symbol_table.rng.gen_range(1..max_jump+1);
        
        for _ in 1..instr_gen_count {
            self.select_random_jump_instr();
        }

        return instr_gen_count;
    }

    fn select_random_alu_instr(&mut self) {

        let dst: u8 = self.symbol_table.get_rand_dst_reg();
        let src: u8 = self.symbol_table.get_rand_src_reg();
        let imm: i32 = self.symbol_table.get_rand_imm();
        let source: Source = self.symbol_table.get_rand_source();

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

    fn select_random_store_instr(&mut self) {

        // "dst" is most likely to be stackpointer (R10) in this context?
        let dst: u8 = self.symbol_table.get_rand_dst_reg();
        let src: u8 = self.symbol_table.get_rand_src_reg();
        let imm: i32 = self.symbol_table.get_rand_imm();
        let offset: i16 = self.symbol_table.get_rand_offset();
        let mem_size: MemSize = self.symbol_table.get_rand_mem_size();

        let instruction = match self.symbol_table.rng.gen_range(0..2) {
            0 => self.prog.store(mem_size).set_dst(dst).set_imm(imm).set_off(offset),
            1 => self.prog.store_x(mem_size).set_dst(dst).set_src(src).set_off(offset),
            _ => unreachable!(),
        };

        instruction.push();
    }

    fn select_random_load_instr(&mut self) {
        
        let dst: u8 = self.symbol_table.get_rand_dst_reg();
        // "src" is most likely to be stackpointer (R10) in this context?
        let src: u8 = self.symbol_table.get_rand_src_reg();
        let imm: i32 = self.symbol_table.get_rand_imm();
        let imm_dw: i32 = self.symbol_table.get_rand_imm();
        let offset: i16 = self.symbol_table.get_rand_offset();
        let mem_size: MemSize = self.symbol_table.get_rand_mem_size();

        match self.symbol_table.rng.gen_range(0..2) {
            0 => {
                match mem_size {
                    MemSize::DoubleWord => {
                        // 128 bit instruction
                        self.prog.load(mem_size).set_dst(dst).set_imm(imm).set_off(offset).push();
                        // This might be a hack, but load word is illegal, but generates a zeroed instruction
                        self.prog.load(MemSize::Word).set_imm(imm_dw).push();
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

    fn select_random_jump_instr(&mut self) {

        let dst: u8 = self.symbol_table.get_rand_dst_reg();
        let src: u8 = self.symbol_table.get_rand_src_reg();
        let imm: i32 = self.symbol_table.get_rand_imm();
        let offset: i16 = self.symbol_table.get_rand_offset();
        let condition: Cond = self.symbol_table.get_rand_jump_condition();
        let source: Source = self.symbol_table.get_rand_source();

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

    fn sequence_mov_imm_to_reg(&mut self) -> i32 {
        // Move an immediate value to a register
        // Useful for initializing - also tracks initialized registers
        let dst: u8 = self.symbol_table.get_rand_dst_reg();
        let imm: i32 = self.symbol_table.get_rand_imm();

        self.symbol_table.initialize_register(dst);

        self.prog.mov(Source::Imm, Arch::X64).set_dst(dst).set_imm(imm).push();

        return 1;
    }

    fn sequence_push_to_stack(&mut self) -> i32 {
        // Move the stack pointer and at store something
        let stack_pointer: u8 = 10;

        let mem_size: MemSize = self.symbol_table.get_rand_mem_size();
        let move_stack_offset: i32 = self.symbol_table.get_mem_size_offset(mem_size);

        let initialized_register_count: usize = self.symbol_table.initialized_register_count();

        if initialized_register_count == 0 {
            // If zero registers has been initialized a instruction is generated anyways
            // You could return here, if this is not desired - it is fuzzing, generate what you want
            let src: u8 = self.symbol_table.get_rand_src_reg();
            let offset: i16 = 0;

            self.prog.store_x(mem_size).set_dst(stack_pointer).set_src(src).set_off(offset).push();
            self.symbol_table.store_from_register(src);
            self.symbol_table.push_value_to_stack(mem_size);
            
            return 1;
        } else {
            let instr_gen_count: usize = self.symbol_table.rng.gen_range(1..initialized_register_count+1);

            for i in 0..instr_gen_count {
                let src: u8 = self.symbol_table.get_init_register(i);
                let offset: i16 = i as i16 * move_stack_offset as i16;

                self.prog.store_x(mem_size).set_dst(stack_pointer).set_src(src).set_off(offset).push();
                self.symbol_table.store_from_register(src);
                self.symbol_table.push_value_to_stack(mem_size);
            }

            return instr_gen_count as i32;
        }
    }

    fn sequence_pop_from_stack(&mut self) -> i32 {
        let stack_pointer: u8 = 10;

        let mem_size: MemSize = self.symbol_table.get_rand_mem_size();
        let move_stack_offset: i32 = self.symbol_table.get_mem_size_offset(mem_size);

        let initialized_register_count: usize = self.symbol_table.initialized_register_count();

        if initialized_register_count == 0 {
            // If zero registers has been initialized a instruction is generated anyways
            // You could return here, if this is not desired - it is fuzzing, generate what you want
            let dst: u8 = self.symbol_table.get_rand_dst_reg();
            let offset: i16 = 0;

            self.prog.load_x(mem_size).set_dst(dst).set_src(stack_pointer).set_off(offset).push();
            self.symbol_table.load_to_register(dst);
            self.symbol_table.pop_value_from_stack(mem_size);
            
            return 1;
        } else {
            let instr_gen_count: usize = self.symbol_table.rng.gen_range(1..initialized_register_count+1);

            for i in 0..instr_gen_count {
                let dst: u8 = self.symbol_table.get_init_register(i);
                let offset: i16 = i as i16 * move_stack_offset as i16;

                self.prog.load_x(mem_size).set_dst(dst).set_src(stack_pointer).set_off(offset).push();
                self.symbol_table.load_to_register(dst);
                self.symbol_table.pop_value_from_stack(mem_size);
            }

            return instr_gen_count as i32;
        }
    }
    
    pub fn init_map(&mut self) {
        // Prepare the stack for "map_lookup_elem"
        //self.prog.mov(Source::Imm, Arch::X64).set_dst(0).set_imm(0).push();

        self.prog.store_x(MemSize::Word).set_dst(10).set_src(0).set_off(-4).push();
        self.prog.mov(Source::Reg, Arch::X64).set_dst(2).set_src(10).push();
        self.prog.add(Source::Imm, Arch::X64).set_dst(2).set_imm(-4).push();

        // Make the call to "map_lookup_elem"
        self.prog.load(MemSize::DoubleWord).set_dst(1).set_imm(-4).push();
        self.prog.load(MemSize::Word).set_dst(0).set_imm(0).push();

        // integer value in 'imm' field of BPF_CALL instruction selects which helper function eBPF program intends to call
        // https://github.com/torvalds/linux/blob/master/include/uapi/linux/bpf.h#L5506
        self.prog.call().set_dst(0).set_src(0).set_off(0).set_imm(0x00_00_00_01).push();
        
        // Verify the map so that we can use it
        //self.prog.jump_conditional(Cond::NotEquals, Source::Imm).set_dst(0).set_imm(0).set_off(1).push();
        //self.prog.exit().push();

        // Initialize two new registers 3 and 4, by reading from map = r0 = map(r1, r2)
        // BPF_LDX_MEM(BPF_DW, this->reg1, BPF_REG_0, 0) this->reg1?
        //self.prog.load_x(MemSize::DoubleWord).set_dst(3).set_src(0).set_off(0).push();
        //self.prog.load_x(MemSize::DoubleWord).set_dst(4).set_src(0).set_off(8).push();

        // generate bounds for new registers 3 and 4, that can be used for operations
        //self.prog.jump_conditional(Cond::Greater, Source::Imm).set_dst(3).set_imm(-8192).set_off(1).push();
        //self.prog.jump_conditional(Cond::Greater, Source::Imm).set_dst(4).set_imm(8192).set_off(1).push();
    }

    pub fn footer(&mut self) {
        // add or sub operation(s) using registers 3 and 4, to ensure mem access
        // r0 = 1 to ensure valid return value
        //self.prog.add(Source::Reg, Arch::X64).set_dst(4).set_src(3).push();
        //self.prog.add(Source::Reg, Arch::X64).set_dst(3).set_src(4).push();

        self.prog.mov(Source::Imm, Arch::X64).set_dst(0).set_imm(1).push();
    }

}
