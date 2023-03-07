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
        let _parsed_prog = vec![b"7f", b"45"];
        _parsed_prog
    }
}