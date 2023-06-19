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
                self.map_body();
                self.map_footer();
            },
            "RandomMaps" => {
                self.init_zero();
                self.init_map();
                self.map_footer();
            },
            "StackSequences" => {
                self.init_zero();
                self.gen_stack_sequences();
            },
            "RuleBreak" => {
                self.init_zero();
                self.gen_rule_break();
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

    fn gen_stack_sequences(&mut self) {
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
                // 5 => self.gen_single_rule_break(),
                _ => unreachable!(),
            };

            instr_gen_count -= generated_count;
        }
    }

    fn gen_rule_break(&mut self) {
        let mut instr_gen_count: i32 = self.symbol_table.get_instr_count();

        loop {
            if instr_gen_count <= 0 {
                break;
            }
    
            let generated_count: i32 = match self.symbol_table.rng.gen_range(0..5) {
                0 => self.gen_single_rule_break(),
                1 => self.random_alu_wrapper(),
                2 => self.random_jump_wrapper(),
                3 => self.random_load_wrapper(),
                4 => self.random_store_wrapper(),
                _ => unreachable!(),
            };
    
            instr_gen_count -= generated_count;
        }
    }

    fn gen_single_rule_break(&mut self) -> i32 {
        // Generate a single "rule break"
        // Idea of this function is to add it into other strategies
        let generated_count: i32 = match self.symbol_table.rng.gen_range(0..2) {
            0 => self.rule_break_write_to_stack_pointer(),
            1 => self.rule_break_jump(),
            _ => unreachable!(),
        };

        generated_count
    }


    fn random_alu_wrapper(&mut self) -> i32{
        let max_alu: i32 = self.symbol_table.get_max_alu_instr();
        // Plus 2 because both of the following num ranges are non-inclusive
        let instr_gen_count: i32 = self.symbol_table.rng.gen_range(1..=max_alu);
        
        for _ in 1..=instr_gen_count {
            self.select_random_alu_instr();
        }

        return instr_gen_count;
    }

    fn random_jump_wrapper(&mut self) -> i32{
        let max_jump: i32 = self.symbol_table.get_max_jump_instr();
        let instr_gen_count: i32 = self.symbol_table.rng.gen_range(1..=max_jump);
        
        for _ in 1..=instr_gen_count {
            self.select_random_jump_instr();
        }

        return instr_gen_count;
    }

    fn random_store_wrapper(&mut self) -> i32{
        let max_store: i32 = self.symbol_table.get_max_store_instr();
        let instr_gen_count: i32 = self.symbol_table.rng.gen_range(1..=max_store);
        
        for _ in 1..=instr_gen_count {
            self.select_random_store_instr();
        }

        return instr_gen_count;
    }

    fn random_load_wrapper(&mut self) -> i32{
        let max_load: i32 = self.symbol_table.get_max_load_instr();
        let instr_gen_count: i32 = self.symbol_table.rng.gen_range(1..=max_load);
        
        for _ in 1..=instr_gen_count {
            self.select_random_load_instr();
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

        let dst: u8 = self.symbol_table.get_stack_pointer();
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
        let src: u8 = self.symbol_table.get_stack_pointer();
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
        let stack_pointer: u8 = self.symbol_table.get_stack_pointer();

        // We are fuzzing after all - adding some random chaos to the stack operation
        // This offset is not tracked, so it doesn't mess with the stack height
        let random_extra_offset: i16 = match self.symbol_table.rng.gen_range(0..5) {
            0..4 => 0,
            4    => self.symbol_table.rng.gen_range(0..513),
            _    => unreachable!(),
        };

        let mem_size: MemSize = self.symbol_table.get_rand_mem_size();

        let initialized_register_count: usize = self.symbol_table.initialized_register_count();

        if initialized_register_count == 0 {
            // If zero registers has been initialized a instruction is generated anyways
            // You could return here, if this is not desired - it is fuzzing, generate what you want
            let src: u8 = self.symbol_table.get_rand_src_reg();
            let offset: i16 = self.symbol_table.stack_to_bottom() as i16;

            self.prog.store_x(mem_size).set_dst(stack_pointer).set_src(src).set_off(offset+random_extra_offset).push();
            self.symbol_table.store_from_register(src);
            self.symbol_table.push_to_stack(mem_size);
            
            return 1;
        } else {
            let instr_gen_count: usize = self.symbol_table.rng.gen_range(1..=initialized_register_count);

            // Below range is non-inclusive, as it is a length used as index
            for i in 0..instr_gen_count {
                let src: u8 = self.symbol_table.get_init_register(i);
                let offset: i16 = self.symbol_table.stack_to_bottom() as i16;

                self.prog.store_x(mem_size).set_dst(stack_pointer).set_src(src).set_off(offset+random_extra_offset).push();
                self.symbol_table.store_from_register(src);
                self.symbol_table.push_to_stack(mem_size);
            }

            return instr_gen_count as i32;
        }
    }

    fn sequence_pop_from_stack(&mut self) -> i32 {
        let stack_pointer: u8 = self.symbol_table.get_stack_pointer();

        // We are fuzzing after all - adding some random chaos to the stack operation
        // This offset is not tracked, so it doesn't mess with the stack height
        let random_extra_offset: i16 = match self.symbol_table.rng.gen_range(0..5) {
            0..4 => 0,
            4    => self.symbol_table.rng.gen_range(0..513),
            _    => unreachable!(),
        };

        let mem_size: MemSize = self.symbol_table.get_rand_mem_size();

        let initialized_register_count: usize = self.symbol_table.initialized_register_count();

        if initialized_register_count == 0 {
            // If zero registers has been initialized a instruction is generated anyways
            // You could return here, if this is not desired - it is fuzzing, generate what you want
            let dst: u8 = self.symbol_table.get_rand_dst_reg();
            let offset: i16 = self.symbol_table.stack_to_bottom() as i16;

            self.prog.load_x(mem_size).set_dst(dst).set_src(stack_pointer).set_off(offset+random_extra_offset).push();
            self.symbol_table.load_to_register(dst);
            self.symbol_table.pop_from_stack(mem_size);
            
            return 1;
        } else {
            let instr_gen_count: usize = self.symbol_table.rng.gen_range(1..=initialized_register_count);

            // Below range is non-inclusive, as it is a length used as index
            for i in 0..instr_gen_count {
                let dst: u8 = self.symbol_table.get_init_register(i);
                let offset: i16 = self.symbol_table.stack_to_bottom() as i16;

                self.prog.load_x(mem_size).set_dst(dst).set_src(stack_pointer).set_off(offset+random_extra_offset).push();
                self.symbol_table.load_to_register(dst);
                self.symbol_table.pop_from_stack(mem_size);
            }

            return instr_gen_count as i32;
        }
    }

    fn rule_break_write_to_stack_pointer(&mut self) -> i32 {
        let dst: u8 = 10;
        let src: u8 = self.symbol_table.get_rand_src_reg();
        let imm: i32 = self.symbol_table.get_rand_imm();
        let source: Source = self.symbol_table.get_rand_source();

        let instruction = self.prog.mov(source, Arch::X64).set_dst(dst);

        match source {
            Source::Imm => instruction.set_imm(imm).push(),
            Source::Reg => instruction.set_src(src).push(),
            _ => unreachable!(),
        };

        1
    }

    fn rule_break_jump(&mut self) -> i32 {
        let offset: i16 = self.symbol_table.gen_rule_break_offset();

        // TODO maybe more than unconditional
        self.prog.jump_unconditional().set_off(offset).push();

        1
    }
    
    fn generate_bounds_jump(&mut self, reg: u8) {

        let min_bound = self.symbol_table.rng.gen_range(-8192..8193);
        self.prog.jump_conditional(Cond::Greater, Source::Imm).set_dst(reg).set_imm(min_bound).set_off(1).push();
    }

    fn select_regs_alu_instr(&mut self) {
        let reg1: u8 = 3;
        let reg2: u8 = 4;

        // Choose a random (ALU) instruction with the two registers
        // TODO swap bytes is missing
        let instruction = match self.symbol_table.rng.gen_range(0..12) {
            0  => self.prog.add(Source::Reg, Arch::X64),
            1  => self.prog.sub(Source::Reg, Arch::X64),
            2  => self.prog.mul(Source::Reg, Arch::X64),
            3  => self.prog.div(Source::Reg, Arch::X64),
            4  => self.prog.modulo(Source::Reg, Arch::X64),
            5  => self.prog.bit_or(Source::Reg, Arch::X64),
            6  => self.prog.bit_xor(Source::Reg, Arch::X64),
            7  => self.prog.bit_and(Source::Reg, Arch::X64),
            8  => self.prog.left_shift(Source::Reg, Arch::X64),
            9  => self.prog.right_shift(Source::Reg, Arch::X64),
            10 => self.prog.signed_right_shift(Source::Reg, Arch::X64),
            11 => self.prog.mov(Source::Reg, Arch::X64),
            _  => unreachable!(),
        };

        // Choose which register is the destination register of the operation and which is the source
        match self.symbol_table.rng.gen_range(reg1..reg2+1) {
            reg1 => { instruction.set_dst(reg1).set_src(reg2).push()
            },
            reg2 => { instruction.set_dst(reg2).set_src(reg1).push()
            },
            _ => unreachable!(),
        };
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
        //self.prog.load_x(MemSize::DoubleWord).set_dst(3).set_src(0).set_off(0).push();
        //self.prog.load_x(MemSize::DoubleWord).set_dst(4).set_src(0).set_off(8).push();

        // generate bounds for new registers 3 and 4, that can be used for operations
        //self.generate_bounds_jump(3);
        //self.generate_bounds_jump(4);
    }

    pub fn map_body(&mut self) {
        // TODO branching operations as well on both reg/imm (symbol table tracks instr count and program size)
        // self.random_jump_wrapper();
        // TODO regs_alu_instr_wrapper(); for instr count gen
        //self.select_regs_alu_instr();
    }

    pub fn map_footer(&mut self) {
        // add or sub operation(s) using registers 3 and 4, to ensure mem access
        //self.prog.add(Source::Reg, Arch::X64).set_dst(4).set_src(3).push();
        //self.prog.add(Source::Reg, Arch::X64).set_dst(3).set_src(4).push();

        // r0 = 1 to ensure valid return value
        self.prog.mov(Source::Imm, Arch::X64).set_dst(0).set_imm(1).push();
    }

}
