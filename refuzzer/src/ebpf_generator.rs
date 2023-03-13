#![allow(unused_imports)]
use rbpf::insn_builder::BpfCode;
use rbpf::insn_builder::{
    Arch,
    Endian,
    Instruction,
    Source,
};

macro_rules! create_function_caller {
    ($function_name:ident, $function_vector:expr) => {
        fn $function_name (generator: &mut EbpfGenerator) {
            for func in $function_vector {
                func(generator);
            }
        }
    };
}

create_function_caller!(call_init_zero, vec![EbpfGenerator::init_zero]);

pub struct EbpfGenerator<'a> {
    seed: u32,
    pub prog: BpfCode,
    config_table: u32,
    configuration: &'a str,
}

impl EbpfGenerator<'_> {
    pub fn new(_seed: u32, _config: &str) -> EbpfGenerator {
        EbpfGenerator { 
            seed: _seed,
            prog: BpfCode::new(),
            config_table: 42,
            configuration: _config,
        }
    }

    pub fn generate_program(&mut self) -> BpfCode{

        match self.configuration {
            "InitZero" => {
                call_init_zero(self);
            },
            _ => {
                //Nothing
            }
        };

        self.prog.exit().push();

        self.prog.clone()
    }

    pub fn init_zero(&mut self) {
        self.prog.mov(Source::Imm, Arch::X64).set_dst(0).set_imm(0x00).push();
    }
}
