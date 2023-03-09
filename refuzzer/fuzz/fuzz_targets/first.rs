#![no_main]

use std::fs;
use std::process::Command;
use std::io::{self, Write};

use arbitrary;
use libfuzzer_sys::fuzz_target;

extern crate refuzzer;

use crate::refuzzer::ebpf_generator::EbpfGenerator;
use crate::refuzzer::elf_parser::ElfParser;

#[derive(arbitrary::Arbitrary, Debug)]
struct FuzzSeedData {
    seed: u32,
}

fuzz_target!(|data: FuzzSeedData| {

    let mut generator = EbpfGenerator::new(data.seed);
    let generated_prog = generator.generate_program();

    let parser = ElfParser::new(generated_prog);
    let parsed_prog = parser.parse_prog();

    fs::write("../obj-files/data.o", parsed_prog).expect("Unable to write file");

    // Backtrace environment variable for debugging.
    // env::set_var("RUST_BACKTRACE", "1");
    let output = Command::new("../ebpf-verifier/check")
                 .args(&["../obj-files/data.o"])
                 .output()
                 .expect("failed to execute process");
    
    // If no errors occur when running, unwrap stdout.
    io::stdout().write_all(&output.stdout).unwrap();
    
    // If any errors occur when running, unwrap stderr instead.
    // io::stderr().write_all(&output.stderr).unwrap();
    
    // Status code
    // println!("output: {}", output.status);

});
