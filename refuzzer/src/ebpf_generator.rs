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
        self.prog.mov(Source::Imm, Arch::X64).set_dst(0x00).set_imm(0).push();
    }

    pub fn init_map(&mut self) {
        // Prepare the stack for "map_lookup_elem"
        //self.prog.mov(Source::Imm, Arch::X64).set_dst(0).set_imm(0).push();

        //BPF_STX_MEM(BPF_W, BPF_REG_10, BPF_REG_0, -4)
        self.prog.store_x(MemSize::DoubleWord).set_dst(0x10).set_src(0x00).set_off(-8).push();

        self.prog.mov(Source::Reg, Arch::X64).set_dst(0x02).set_src(0x10).push();
        self.prog.add(Source::Imm, Arch::X64).set_dst(0x02).set_imm(-8).push();

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
        let test: i32 = 1;
        let map_fd_ptr: *const i32 = &test;
        let map_fd_str = format!("{:p}", map_fd_ptr);

        // trim hex pointer address 0x
        let map_fd_str_nox = map_fd_str.trim_start_matches("0x");
        let map_fd_dec = i32::from_str_radix(map_fd_str_nox, 16);

        // convert radix result through debug formatter: Ok(i32), trim Ok()
        let map_fd_dec_ok = format!("{:?}", map_fd_dec);
        let map_fd_dec_nok1 = map_fd_dec_ok.trim_start_matches("Ok(");
        let map_fd_dec_nok = &map_fd_dec_nok1[0..map_fd_dec_nok1.len()-1];

        // overflow i32 -> i32?
        //let map_fd: i32 = map_fd_dec_nok.parse::<i32>().unwrap();

        //println!("{}", map_fd);
        self.prog.mov(Source::Imm, Arch::X64).set_dst(0x01).set_imm(1).push();

        //BPF_RAW_INSN(BPF_JMP | BPF_CALL, 0, 0, 0, BPF_FUNC_map_lookup_elem)
        // define BPF_RAW_INSN(CODE, DST, SRC, OFF, IMM)
        //     BPF_JMP | BPF_CALL ? (bpf_common.h)
        //     BPF_FUNC_map_lookup_elem helper func? bpf_map_lookup_elem
        // https://github.com/torvalds/linux/blob/master/include/uapi/linux/bpf.h#L5727
        // integer value in 'imm' field of BPF_CALL instruction selects which helper function eBPF program intends to call
        // https://github.com/torvalds/linux/blob/master/include/uapi/linux/bpf.h#L5506
        // 1 = map_lookup_elem 0x11_22_33_44 = 44, 33, 22, 11 (little endianness)
        self.prog.call().set_dst(0x00).set_src(0x01).set_off(0).set_imm(0x00_00_00_01).push();
        
        // Verify the map so that we can use it
        self.prog.jump_conditional(Cond::NotEquals, Source::Imm).set_dst(0x00).set_imm(1).push();
        self.prog.exit().push();

        // Initialize two registers by reading from map?
        // BPF_LDX_MEM(BPF_DW, this->reg1, BPF_REG_0, 0) this->reg1?
        self.prog.load_x(MemSize::DoubleWord).set_dst(0x01).set_src(0x00).set_off(0).push();
        self.prog.load_x(MemSize::DoubleWord).set_dst(0x02).set_src(0x00).set_off(8).push();

    }
}
