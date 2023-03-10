#![allow(unused_imports)]

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

        // Create file we want verify with "./check" from PREVAIL
        let name = "../obj-files/data.o";
        let file = File::create(Path::new(name))?;

        // Set target
        let target = Triple {
            architecture: Architecture::X86_64,
            vendor: Vendor::Unknown,
            operating_system: OperatingSystem::Linux,
            environment: Environment::Unknown,
            binary_format: BinaryFormat::Elf,
        };

        // Faerie obj-file builder
        let mut obj = ArtifactBuilder::new(target)
                      .name(name.to_owned())
                      .finish();

        // PREVAIL looks for ".text" section
        let declarations: Vec<(&'static str, Decl)> = vec![
            (".text", Decl::section(SectionKind::Text).into()),
        ];

        obj.declarations(declarations.into_iter())?;

        // First parse generated eBPF into bytes
        let byte_code = &mut self.generated_prog.into_bytes();

        // Then define the eBPF program under ".text"
        obj.define(".text", byte_code.to_vec())?;

        // Write to the path
        obj.write(file)?;

        // Return () if everything went well
        Ok(())
    }
}
