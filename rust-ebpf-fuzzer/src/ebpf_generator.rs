use rbpf::insn_builder::BpfCode;

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
        let prog = BpfCode::new();

        // Generate program her
        
        prog
    }
}
