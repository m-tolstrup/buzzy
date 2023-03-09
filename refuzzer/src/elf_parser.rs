use rbpf::insn_builder::{
    BpfCode,
    IntoBytes,
};

pub struct ElfParser {
    pub generated_prog: BpfCode,
}

impl ElfParser {
    pub fn new(_generated_prog: BpfCode) -> ElfParser {
        ElfParser { 
            generated_prog: _generated_prog,
        }
    }

    pub fn parse_prog(self) -> Vec<u8>{
        // ELF Format magic bytes for preprending on generated program
        // 7f = magic number, 45 4c 46 = elf
        // 7f 45 4c 46 02 01 01 00 00 00 00 00 00 00 00 00
        let mut parsed_prog: Vec<u8> = vec![127,69,76,70,2,1,1,0,0,0,0,0,0,0,0,0];
        
        // ELF header from bpf_lxc.o
        // 01 00 f7 00 01 00 00 00 00 00 00 00 00 00 00 00
        parsed_prog.append(&mut vec![1,0,247,0,1,0,0,0,0,0,0,0,0,0,0,0]);

        // File header                               x,x,x,x
        parsed_prog.append(&mut vec![0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]);

        // Program header                                      x,x
        parsed_prog.append(&mut vec![0,0,0,0,64,0,0,0,0,0,64,0,0,0,0,0]);

        parsed_prog.append(&mut self.generated_prog.into_bytes().to_vec());

        parsed_prog
    }
}
