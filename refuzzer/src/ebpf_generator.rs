#![allow(unused_imports)]
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
            "InitHeader" => {
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
        self.prog.mov(Source::Imm, Arch::X64).set_dst(0).set_imm(0).push();
    }

    pub fn init_map(&mut self) {
        // Prepare the stack for "map_lookup_elem"
        // BPF_MOV64_IMM(BPF_REG_0, 0)
        self.prog.mov(Source::Imm, Arch::X64).set_dst(0).set_imm(0).push();

        //BPF_STX_MEM(BPF_W, BPF_REG_10, BPF_REG_0, -4)
        self.prog.store_x(MemSize::Word).set_dst(10).set_src(0).set_off(-4).push();

        // BPF_MOV64_REG(BPF_REG_2, BPF_REG_10)
        self.prog.mov(Source::Reg, Arch::X64).set_dst(2).set_src(10).push();

        // BPF_ALU64_IMM(BPF_ADD, BPF_REG_2, -4)
        self.prog.add(Source::Imm, Arch::X64).set_dst(2).set_imm(-4).push();

        // A bit of trolling
        self.prog.mov(Source::Imm, Arch::X64).set_dst(3).set_imm(1).push();
        self.prog.mov(Source::Reg, Arch::X64).set_dst(1).set_src(3).push();

        // Make the call to "map_lookup_elem"
        // BPF_LD_MAP_FD(BPF_REG_1, BPF_TRIAGE_MAP_FD)
        // self.prog.mov(Source::Imm, Arch::X64).set_dst(1).set_imm(1).push();

        // BPF_RAW_INSN(BPF_JMP | BPF_CALL, 0, 0, 0, BPF_FUNC_map_lookup_elem)
        // integer value in 'imm' field of BPF_CALL instruction selects which helper function eBPF program intends to call
        self.prog.call().set_dst(0).set_src(0).set_off(0).set_imm(1).push();

        // Verify the map so that we can use it
        // BPF_JMP_IMM(BPF_JNE, BPF_REG_0, 0, 1)
        self.prog.jump_conditional(Cond::NotEquals, Source::Imm).set_dst(0).set_imm(1).push();
        //     
        //BPF_RAW_INSN(BPF_JMP | BPF_CALL, 0, 0, 0, BPF_FUNC_map_lookup_elem)
        // define BPF_RAW_INSN(CODE, DST, SRC, OFF, IMM)
        //     BPF_JMP | BPF_CALL ? (bpf_common.h)
        //     BPF_FUNC_map_lookup_elem helper func? bpf_map_lookup_elem
        // https://github.com/torvalds/linux/blob/master/include/uapi/linux/bpf.h#L5727
        // integer value in 'imm' field of BPF_CALL instruction selects which helper function eBPF program intends to call
        // https://github.com/torvalds/linux/blob/master/include/uapi/linux/bpf.h#L5506
        // 1 = map_lookup_elem? 0x11_22_33_44 = 44, 33, 22, 11 (little endianness)
    }
}
