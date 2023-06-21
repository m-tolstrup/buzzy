#![allow(unused_imports)]

/* USED TO GATHER PERFORMANCE RESULTS WHEN GENERATING EBPF PROGRAMS CONSISTING OF COMPLETLY RANDOM BYTES */

use std::fs::File;
use std::path::Path;
use std::str::FromStr;

use target_lexicon::{
    Architecture,
    BinaryFormat,
    Environment,
    OperatingSystem,
    Triple,
    Vendor
};

use faerie::{
    ArtifactBuilder,
    Decl,
    SectionKind,
    Link,
    Reloc,
};

use crate::common::Instruction;

pub struct RandomBytesParser {
    pub prog: Vec<Instruction>,
}

impl RandomBytesParser {
    pub fn new(_prog: Vec<Instruction>) -> RandomBytesParser {
        RandomBytesParser { 
            prog: _prog,
        }
    }

    pub fn parse_prog(self) -> anyhow::Result<()> {

        // Create file we want verify with "./check" from PREVAIL
        let name = "obj-files/data.o";
        let file = File::create(Path::new(name))?;

        // Set target
        let target = Triple {
            architecture: Architecture::Bpfel,
            vendor: Vendor::Unknown,
            operating_system: OperatingSystem::Linux,
            environment: Environment::Unknown,
            binary_format: BinaryFormat::Elf,
        };
        
        // Faerie obj-file builder
        let mut obj = ArtifactBuilder::new(target)
                      .name(name.to_owned())
                      .finish();

        // PREVAIL looks for ".text" section and "maps" section relocations
        let declarations: Vec<(&'static str, Decl)> = vec![
            (".text", Decl::section(SectionKind::Text).with_loaded(true).into())
        ];

        obj.declarations(declarations.into_iter())?;

        let mut random_byte_instr: Vec<u8> = vec![];

        if self.prog.len() > 0 {
            for instr in self.prog {
                random_byte_instr.append(&mut instr.into_bytes());
            }
        }

        // Then define the eBPF program under ".text"
        obj.define(".text", random_byte_instr)?;

        // Write to the path
        obj.write(file)?;

        // Return () if everything went well
        Ok(())
    }
}
