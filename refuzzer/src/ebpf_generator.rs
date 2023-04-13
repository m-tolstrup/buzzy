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

    fn random_instructs(&mut self) {

        // Always initialize zero - lets more program through verifier
        self.prog.mov(Source::Imm, Arch::X64).set_dst(0).set_imm(0).push();
        
        loop {
            if self.config_table.random_instr_count == 0 {
                break;
            }

            self.select_random_instr();
            
            self.config_table.random_instr_count -= 1;
        }
    }

    fn select_random_instr(&mut self) {

        let dst: u8 = self.config_table.get_rand_dst_reg();
        let src: u8 = self.config_table.get_rand_src_reg();
        let imm: i32 = self.config_table.get_rand_imm();

        // Select the source type, either register or immediate
        let source: Source = match rand::thread_rng().gen_range(0..2) {
            0 => Source::Imm,
            1 => Source::Reg,
            _ => !unreachable!(),
        };

        // Choose a random instruction and set the source
        let instruction = match rand::thread_rng().gen_range(0..4) {
            0 => self.prog.add(source, Arch::X64).set_dst(dst),
            1 => self.prog.sub(source, Arch::X64).set_dst(dst),
            2 => self.prog.mul(source, Arch::X64).set_dst(dst),
            3 => self.prog.div(source, Arch::X64).set_dst(dst),
            _ => !unreachable!(),
        };

        // Then, depending on the source type, set the value of the source and push it
        match source {
            Source::Imm => instruction.set_imm(imm).push(),
            Source::Reg => instruction.set_src(src).push(),
            _ => !unreachable!(),
        };
    }

}
