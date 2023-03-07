use rbpf::insn_builder::BpfCode;

pub struct ElfParser {
    pub prog: BpfCode,
}

impl ElfParser {
    pub fn new(generated_prog: BpfCode) -> ElfParser {
        ElfParser { 
            prog: generated_prog,
        }
    }

    pub fn parse_prog(self) -> Vec<u8>{
        // ELF Format magic bytes for preprending FuzzData
        // 7f = magic number, 45 4c 46 = elf

        // ELF header from bpf_lxc.co
        //7f 45 4c 46 02 01 01 00 00 00 00 00 00 00 00 00
        //01 00 f7 00 01 00 00 00 00 00 00 00 00 00 00 00
        //let mut elf_magic_bytes: Vec<u8> = vec![127,69,76,70,2,1,1,0,0,0,0,0,0,0,0,0,
        //                                        1,0,247,0,1,0,0,0,0,0,0,0,0,0,0,0];

        // ELF header from wiki
        //7f 45 4c 46 02 01 01 00 00 00 00 00 00 00 00 00
        //02 00 3e 00 01 00 00 00 c5 48 40 00 00 00 00 00
        let mut _parsed_prog: Vec<u8> = vec![127,69,76,70,2,1,1,0,0,0,0,0,0,0,0,0,
                                            2,0,62,0,1,0,0,0,197,72,64,0,0,0,0,0];


        // Parse BpfCode to u8 here

        _parsed_prog
    }
}
