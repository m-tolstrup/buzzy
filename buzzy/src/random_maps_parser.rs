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

use crate::common::Map;

pub struct RandomMapsParser<'a> {
    pub generated_prog: BpfCode,
    strategy: &'a str,
    maps: Vec<Map>,
}

impl RandomMapsParser<'_> {
    pub fn new(_generated_prog: BpfCode, _strategy: &str, _maps: Vec<Map>) -> RandomMapsParser {
        RandomMapsParser { 
            generated_prog: _generated_prog,
            strategy: _strategy,
            maps: _maps,
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
        declarations.append(&mut vec![
            ("maps", Decl::data().writable().into()),
        ]);

        obj.declarations(declarations.into_iter())?;

        // First parse generated eBPF into bytes
        let byte_code = &mut self.generated_prog.into_bytes();

        // Then define the eBPF program under ".text"
        obj.define(".text", byte_code.to_vec())?;
        
        // TODO: 4 +- 2 random u8
        let maps = self.maps;

        let mut random_map: Vec<u8> = vec![];

        if maps.len() > 0 {
            for attr in maps {
                random_map.append(&mut attr.into_bytes());
            }
        }

        obj.define("maps", random_map);

        obj.link_with(
            Link { from: ".text", to: "maps", at: (4*8) },
            Reloc::Debug { size: 8, addend: 0x00 },
        )?;
        

        // Write to the path
        obj.write(file)?;

        // Return () if everything went well
        Ok(())
    }
}
