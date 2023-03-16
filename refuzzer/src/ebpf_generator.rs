#![allow(unused_imports)]
use rbpf::insn_builder::BpfCode;
use rbpf::insn_builder::{
    Arch,
    Endian,
    Instruction,
    Source,
    Cond,
    MemSize,
};

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
                self.init_zero();
            },
            "InitHeader" => {
                self.init_zero();
                self.init_map();
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

    pub fn init_map(&mut self) {
        // Prepare the stack for "map_lookup_elem"
        //self.prog.mov(Source::Imm, Arch::X64).set_dst(0).set_imm(0x00).push();

        //BPF_STX_MEM(BPF_W, BPF_REG_10, BPF_REG_0, -4) -4? (.set_off(-4) )
        self.prog.store_x(MemSize::Word).set_dst(10).set_src(0).push();

        self.prog.mov(Source::Reg, Arch::X64).set_dst(2).set_src(10).push();
        self.prog.add(Source::Imm, Arch::X64).set_dst(2).set_imm(-4).push();

        // Make the call to "map_lookup_elem"
        // TODO manually insert load map bytecode*
        // TODO implement map bytecode translate in rbpf
        // https://github.com/torvalds/linux/blob/master/include/uapi/linux/bpf_common.h
        //BPF_LD_MAP_FD(BPF_REG_1, BPF_TRIAGE_MAP_FD)
        // define BPF_LD_MAP_FD(DST, MAP_FD) r1 = map_fd
        //     BPF_LD_IMM64_RAW(DST, BPF_PSEUDO_MAP_FD, MAP_FD)
        //         DST = 1
        //         BPF_PSEUDO_MAP_FD = BPF_TRIAGE_MAP_FD = 1
        //         MAP_FD = ? (placeholder = set_src(1))
        self.prog.mov(Source::Reg, Arch::X64).set_dst(1).set_src(1).push();
        //     
        //BPF_RAW_INSN(BPF_JMP | BPF_CALL, 0, 0, 0, BPF_FUNC_map_lookup_elem)
        // define BPF_RAW_INSN(CODE, DST, SRC, OFF, IMM)
        //     BPF_JMP | BPF_CALL ? (bpf_common.h)
        //     BPF_FUNC_map_lookup_elem helper func?

        // Verify the map so that we can use it
        self.prog.jump_conditional(Cond::NotEquals, Source::Imm).set_dst(0).set_imm(1).push();
        self.prog.exit().push();

        // Initialize two registers by reading from map?
        // BPF_LDX_MEM(BPF_DW, this->reg1, BPF_REG_0, 0) this->reg1?
        self.prog.load_x(MemSize::DoubleWord).set_dst(1).set_src(0).set_off(0).push();
        self.prog.load_x(MemSize::DoubleWord).set_dst(2).set_src(0).set_off(8).push();

    }
}
