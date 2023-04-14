#![allow(unused_imports)]
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

use crate::config_table::ConfigTable;

pub struct EbpfGenerator<'a> {
    prog: BpfCode,
    config_table: ConfigTable,
    strategy: &'a str,
}

impl EbpfGenerator<'_> {
    pub fn new(_seed: u32, _strategy: &str) -> EbpfGenerator {
        EbpfGenerator { 
            prog: BpfCode::new(),
            config_table: ConfigTable::new(_seed),
            strategy: _strategy,
        }
    }

    pub fn generate_program(&mut self) -> BpfCode{

        match self.strategy {
            "InitZero" => {
                self.init_zero();
            },
            "Random" => {
                self.random_instructs();
            },
            _ => {
                //Nothing
            }
        };

        // Always push exit instruction
        self.prog.exit().push();

        self.prog.clone()
    }

    fn init_zero(&mut self) {
        self.prog.mov(Source::Imm, Arch::X64).set_dst(0).set_imm(0).push();
    }

    fn random_instructs(&mut self) {

        // Always initialize zero - lets more programs through the verifier
        self.prog.mov(Source::Imm, Arch::X64).set_dst(0).set_imm(0).push();
        
        loop {
            if self.config_table.random_instr_count == 0 {
                break;
            }

            // Weighted to match number of available instructions
            match rand::thread_rng().gen_range(0..20) {
                0..13  => self.select_random_alu_instr(),
                13..15 => self.select_random_store_instr(),
                15..19 => self.select_random_load_instr(),
                19..20 => self.select_random_jump_instr(),
                _      => !unreachable!(),
            }

            self.config_table.random_instr_count -= 1;
        }
    }

    fn select_random_alu_instr(&mut self) {

        let dst: u8 = self.config_table.get_rand_dst_reg();
        let src: u8 = self.config_table.get_rand_src_reg();
        let imm: i32 = self.config_table.get_rand_imm();

        // Select the source type
        let source: Source = match rand::thread_rng().gen_range(0..2) {
            0 => Source::Imm,
            1 => Source::Reg,
            _ => !unreachable!(),
        };

        // Choose a random (ALU) instruction and set the destination register
        // TODO swap bytes is missing
        let instruction = match rand::thread_rng().gen_range(0..13) {
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
            _  => !unreachable!(),
        };

        // Then, depending on the source type, set the value of the source and push it
        match source {
            Source::Imm => instruction.set_imm(imm).push(),
            Source::Reg => instruction.set_src(src).push(),
            _ => !unreachable!(),
        };
    }

    pub fn select_random_store_instr(&mut self) {

        // "dst" is most likely to be stackpointer (R10) in this context?
        let dst: u8 = self.config_table.get_rand_dst_reg();
        let src: u8 = self.config_table.get_rand_src_reg();
        let imm: i32 = self.config_table.get_rand_imm();
        let offset: i16 = self.config_table.get_rand_offset();

        let mem_size: MemSize = match rand::thread_rng().gen_range(0..4) {
            0 => MemSize::Byte,
            1 => MemSize::HalfWord,
            2 => MemSize::Word,
            3 => MemSize::DoubleWord,
            _ => !unreachable!(),
        };

        let instruction = match rand::thread_rng().gen_range(0..2) {
            0 => self.prog.store(mem_size).set_dst(dst).set_imm(imm).set_off(offset),
            1 => self.prog.store_x(mem_size).set_dst(dst).set_src(src).set_off(offset),
            _ => !unreachable!(),
        };

        instruction.push();
    }

    pub fn select_random_load_instr(&mut self) {
        
        let dst: u8 = self.config_table.get_rand_dst_reg();
        // "src" is most likely to be stackpointer (R10) in this context?
        let src: u8 = self.config_table.get_rand_src_reg();
        let imm: i32 = self.config_table.get_rand_imm();
        let offset: i16 = self.config_table.get_rand_offset();

        let mem_size: MemSize = match rand::thread_rng().gen_range(0..4) {
            0 => MemSize::Byte,
            1 => MemSize::HalfWord,
            2 => MemSize::Word,
            3 => MemSize::DoubleWord,
            _ => !unreachable!(),
        };

        // TODO maybe delete abs and ind? Kernel docs says they are legacy
        let instruction = match rand::thread_rng().gen_range(0..4) {
            0 => self.prog.load(mem_size).set_dst(dst).set_imm(imm).set_off(offset),
            1 => self.prog.load_abs(mem_size).set_dst(dst).set_src(src).set_off(offset),
            2 => self.prog.load_ind(mem_size).set_dst(dst).set_src(src).set_off(offset),
            3 => self.prog.load_x(mem_size).set_dst(dst).set_src(src).set_off(offset),
            _ => !unreachable!(),
        };

        instruction.push();
    }

    pub fn select_random_jump_instr(&mut self) {

        let dst: u8 = self.config_table.get_rand_dst_reg();
        let src: u8 = self.config_table.get_rand_src_reg();
        let imm: i32 = self.config_table.get_rand_imm();
        let offset: i16 = self.config_table.get_rand_offset();

        let condition: Cond = match rand::thread_rng().gen_range(0..11) {
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
            _  => !unreachable!(),
        };

        let source: Source = match rand::thread_rng().gen_range(0..2) {
            0 => Source::Imm,
            1 => Source::Reg,
            _ => !unreachable!(),
        };

        // Weighted to match number of jump instructions
        let instruction = match rand::thread_rng().gen_range(0..12) {
            0..1  => self.prog.jump_unconditional().set_dst(dst).set_src(src).set_imm(imm).set_off(offset),
            1..12 => self.prog.jump_conditional(condition, source).set_dst(dst),
            _     => !unreachable!(),
        };

        match source {
            Source::Imm => instruction.set_imm(imm).set_off(offset).push(),
            Source::Reg => instruction.set_src(src).set_off(offset).push(),
            _           => !unreachable!(),
        };
    }

}
