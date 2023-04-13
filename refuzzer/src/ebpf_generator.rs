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

        self.prog.exit().push();

        self.prog.clone()
    }

    fn random_instructs(&mut self) {

        if self.config_table.random_instr_count == 0 {
            self.prog.mov(Source::Imm, Arch::X64).set_dst(0).set_imm(0).push();
        }
        
        loop {
            if self.config_table.random_instr_count == 0 {
                break;
            }

            self.select_random_instr();
            
            self.config_table.random_instr_count -= 1;
        }
    }

    fn select_random_instr(&mut self) {
        match rand::thread_rng().gen_range(0..1) {
            0 => self.prog.mov(Source::Imm, Arch::X64).set_dst(0).set_imm(0).push(),
            _ => &mut self.prog,
        };
    }

}
