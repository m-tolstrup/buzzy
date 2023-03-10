#![allow(unused_imports)]
use rbpf::insn_builder::BpfCode;
use rbpf::insn_builder::{
    Arch,
    Endian,
    Instruction,
    Source,
};

pub struct EbpfGenerator {
    pub seed: u32,
}

impl EbpfGenerator {
    pub fn new(_seed: u32) -> EbpfGenerator {
        EbpfGenerator { 
            seed: _seed,
        }
    }

    pub fn generate_program(&mut self) -> BpfCode {
        let mut program = BpfCode::new();

        program.mov(Source::Imm, Arch::X64).set_dst(0).set_imm(0x00).push()
               .exit().push();

        program
    }
}
