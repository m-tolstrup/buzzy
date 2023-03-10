#![allow(unused_imports)]

use std::fs::File;
use std::path::Path;
use std::str::FromStr;

use target_lexicon::triple;

use faerie::{
    ArtifactBuilder,
    Decl,
};

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

    pub fn parse_prog(self) -> anyhow::Result<()> {

        let name = "../obj-files/data.o";
        let file = File::create(Path::new(name))?;
        let mut obj = ArtifactBuilder::new(triple!("x86_64-unknown-unknown-unknown-elf"))
                      .name(name.to_owned())
                      .finish();

        obj.declarations([
            ("func", Decl::function().into()),
        ].iter().cloned())?;

        let byte_code = &mut self.generated_prog.into_bytes();

        obj.define("func", byte_code.to_vec())?;

        obj.write(file)?;

        Ok(())
    }
}
