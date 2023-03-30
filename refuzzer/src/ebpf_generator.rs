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
                self.scannell_header();
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
        //self.prog.mov(Source::Imm, Arch::X64).set_dst(0).set_imm(0).push();

        self.prog.store_x(MemSize::Word).set_dst(10).set_src(0).set_off(-4).push();

        self.prog.mov(Source::Reg, Arch::X64).set_dst(2).set_src(10).push();
        self.prog.add(Source::Imm, Arch::X64).set_dst(2).set_imm(-4).push();

        // Make the call to "map_lookup_elem"
        //BPF_LD_MAP_FD(BPF_REG_1, BPF_TRIAGE_MAP_FD)
        self.prog.mov(Source::Imm, Arch::X64).set_dst(1).set_imm(1).push();

        // integer value in 'imm' field of BPF_CALL instruction selects which helper function eBPF program intends to call
        // https://github.com/torvalds/linux/blob/master/include/uapi/linux/bpf.h#L5506
        self.prog.call().set_dst(0).set_src(1).set_off(0).set_imm(0x00_00_00_01).push();
        
        // Verify the map so that we can use it
        self.prog.jump_conditional(Cond::NotEquals, Source::Imm).set_dst(0).set_imm(0).set_off(1).push();
        self.prog.exit().push();

        // Initialize two registers by reading from map?
        // BPF_LDX_MEM(BPF_DW, this->reg1, BPF_REG_0, 0) this->reg1?
        self.prog.load_x(MemSize::DoubleWord).set_dst(1).set_src(0).set_off(0).push();
        self.prog.load_x(MemSize::DoubleWord).set_dst(2).set_src(0).set_off(8).push();

    }

    pub fn scannell_header (&mut self) {
        // mov64 r2, 0x0
        self.prog.mov(Source::Imm, Arch::X64).set_dst(2).set_imm(0).push();
        // stxdw [r10-0x10], r2
        self.prog.store_x(MemSize::DoubleWord).set_dst(10).set_src(2).set_off(-16).push();
        // mov64 r1, r2
        self.prog.mov(Source::Reg, Arch::X64).set_dst(1).set_src(2).push();
        // call 0xffffffff (8510 0000 ffff ffff)
        // TODO translate functions in rBPF like clang does?
        self.prog.call().set_dst(0).set_src(1).set_off(0).set_imm(0x00_00_00_01).push();
        // ldxdw r3 [r10-0x10]
        self.prog.load_x(MemSize::DoubleWord).set_dst(3).set_src(10).set_off(-16).push();
        // mov64 r2, 0xa
        self.prog.mov(Source::Imm, Arch::X64).set_dst(2).set_imm(10).push();
        // stxdw [r10-0x30], r2
        self.prog.store_x(MemSize::DoubleWord).set_dst(10).set_src(2).set_off(-48).push();
        // lddw r4, 0xfffffffc
        // TODO - missing last ff (can because of i32 not u32)
        self.prog.load(MemSize::DoubleWord).set_dst(4).set_imm(1).push();
        // stxdw [r10-0x28], r4
        // TODO DOES NOT SHOW UP???
        self.prog.store_x(MemSize::DoubleWord).set_dst(10).set_src(4).set_off(-40).push();
        self.prog.store_x(MemSize::DoubleWord).set_dst(10).set_src(4).set_off(-40).push();
        // mov64 r1, r3
        self.prog.mov(Source::Reg, Arch::X64).set_dst(1).set_src(3).push();
        // call 0xffffffff (8510 0000 ffff ffff)
        self.prog.call().set_dst(0).set_src(1).set_off(0).set_imm(0x00_00_00_01).push();
        // ldxdw r2 [r10-0x30]
        self.prog.load_x(MemSize::DoubleWord).set_dst(2).set_src(10).set_off(-48).push();
        // mov64 r1, 0x2
        self.prog.mov(Source::Imm, Arch::X64).set_dst(1).set_imm(2).push();
        // stxdw [r10-0x20], r1
        self.prog.store_x(MemSize::DoubleWord).set_dst(10).set_src(1).set_off(-32).push();
        // call 0xffffffff (8510 0000 ffff ffff)
        self.prog.call().set_dst(0).set_src(1).set_off(0).set_imm(0x00_00_00_01).push();
        // ldxdw r2 [r10-0x30]
        self.prog.load_x(MemSize::DoubleWord).set_dst(3).set_src(10).set_off(-40).push();
        // ldxdw r2 [r10-0x30]
        self.prog.load_x(MemSize::DoubleWord).set_dst(2).set_src(10).set_off(-32).push();
        // ldxdw r2 [r10-0x30]
        self.prog.load_x(MemSize::DoubleWord).set_dst(1).set_src(10).set_off(-16).push();
        // call 0xffffffff (8510 0000 ffff ffff)
        self.prog.call().set_dst(0).set_src(1).set_off(0).set_imm(0x00_00_00_01).push();
        // mov64 r2, 0x1
        self.prog.mov(Source::Imm, Arch::X64).set_dst(2).set_imm(1).push();
        // stxdw [r10-0x18], r2
        self.prog.store_x(MemSize::DoubleWord).set_dst(10).set_src(2).set_off(-24).push();
        // mov64 r1, r2
        self.prog.mov(Source::Reg, Arch::X64).set_dst(1).set_src(2).push();
        // call 0xffffffff (8510 0000 ffff ffff)
        self.prog.call().set_dst(0).set_src(1).set_off(0).set_imm(0x00_00_00_01).push();
        // ldxdw r5 [r10-0x18]
        self.prog.load_x(MemSize::DoubleWord).set_dst(5).set_src(10).set_off(-24).push();
        // ldxdw r4 [r10-0x10]
        self.prog.load_x(MemSize::DoubleWord).set_dst(4).set_src(10).set_off(-16).push();
        // mov64 r1, 0x85
        self.prog.mov(Source::Imm, Arch::X64).set_dst(1).set_imm(0x85).push();
        // mov64 r2, r4
        self.prog.mov(Source::Reg, Arch::X64).set_dst(2).set_src(4).push();
        // mov64 r3, r4
        self.prog.mov(Source::Reg, Arch::X64).set_dst(3).set_src(4).push();
        // call 0xffffffff (8510 0000 ffff ffff)
        self.prog.call().set_dst(0).set_src(1).set_off(0).set_imm(0x00_00_00_01).push();
        // ldxdw r4 [r10-0x18]
        self.prog.load_x(MemSize::DoubleWord).set_dst(4).set_src(10).set_off(-24).push();
        // ldxdw r3 [r10-0x10]
        self.prog.load_x(MemSize::DoubleWord).set_dst(3).set_src(10).set_off(-16).push();
        // mov64 r1, 0x50
        self.prog.mov(Source::Imm, Arch::X64).set_dst(1).set_imm(0x50).push();
        // mov64 r2, r3
        self.prog.mov(Source::Reg, Arch::X64).set_dst(2).set_src(3).push();
        // call 0xffffffff (8510 0000 ffff ffff)
        self.prog.call().set_dst(0).set_src(1).set_off(0).set_imm(0x00_00_00_01).push();
        // call 0xffffffff (8510 0000 ffff ffff)
        self.prog.call().set_dst(0).set_src(1).set_off(0).set_imm(0x00_00_00_01).push();
        // ldxdw r0 [r10-0x4]
        self.prog.load_x(MemSize::Word).set_dst(0).set_src(10).set_off(-4).push();
    }
}
