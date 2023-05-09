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
    Link,
    Reloc,
};

use rbpf::insn_builder::{
    BpfCode,
    IntoBytes,
};

pub struct ElfParser<'a> {
    pub generated_prog: BpfCode,
    strategy: &'a str,
}

impl ElfParser<'_> {
    pub fn new(_generated_prog: BpfCode, _strategy: &str) -> ElfParser {
        ElfParser { 
            generated_prog: _generated_prog,
            strategy: _strategy,
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
        let mut declarations: Vec<(&'static str, Decl)> = vec![
            (".text", Decl::section(SectionKind::Text).with_loaded(true).into())
        ];
        if self.strategy == "MapHeader" {
            declarations.append(&mut vec![
                ("maps", Decl::data().writable().into()),
            ]);
        }

        obj.declarations(declarations.into_iter())?;

        // First parse generated eBPF into bytes
        let byte_code = &mut self.generated_prog.into_bytes();

        // Then define the eBPF program under ".text"
        obj.define(".text", byte_code.to_vec())?;

        if self.strategy == "MapHeader"{
            //type = BPF_MAP_TYPE_ARRAY = 2
            //key size   = 4 (8?)
            //value size = size of map = 8192 (0x20, 0x00)
            //max entry  = 1
            //"map in map" = zeroed
            obj.define("maps", vec![0x02, 0x00, 0x00, 0x00,
                                    0x04, 0x00, 0x00, 0x00,
                                    0x00, 0x20, 0x00, 0x00,
                                    0x01, 0x00, 0x00, 0x00,
                                    0x00, 0x00, 0x00, 0x00,
                                    //0x00, 0x00, 0x00, 0x00,
                                    //0x00, 0x00, 0x00, 0x00
                                    ])?;

            obj.link_with(
                Link { from: ".text", to: "maps", at: (4*8) },
                Reloc::Debug { size: 8, addend: 0x00 },
            )?;
        }

        // Write to the path
        obj.write(file)?;

        // Return () if everything went well
        Ok(())
    }
}
