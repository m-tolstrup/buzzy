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

        program.add(Source::Imm, Arch::X64).set_dst(1).set_imm(0x605).push()
               .mov(Source::Imm, Arch::X64).set_dst(2).set_imm(0x32).push()
               .negate(Arch::X64).set_dst(2).push()
               .mov(Source::Reg, Arch::X64).set_src(0).set_dst(1).push()
               .exit().push();

        program
    }
}
