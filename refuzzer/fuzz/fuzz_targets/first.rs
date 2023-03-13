#![no_main]
#![allow(unused_imports)]

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
    // Generate a program - fuzzed structure provides randomness
    let mut generator = EbpfGenerator::new(data.seed);
    let generated_prog = generator.generate_program();

    // Pass it to the parser and parse it
    let parser = ElfParser::new(generated_prog);
    let _ = match parser.parse_prog() {
        Ok(_) => {
            // Do nothing, everything went Ok
        },
        Err(_) => {
            // Return early, as something went wrong when parsing to .o-file
            return;
        }
    };

    // Backtrace environment variable for debugging.
    // env::set_var("RUST_BACKTRACE", "1");

    // Verify the eBPF program with PREVAIL
    // let output = Command::new("../ebpf-verifier/check")
    //              .args(&["../obj-files/data.o"])
    //              .output()
    //              .expect("failed to execute process");
    
    // Status code
    // println!("output: {}", output.status);

    // If no errors occur when running, unwrap stdout.
    // io::stdout().write_all(&output.stdout).unwrap();
    
    // If any errors occur when running, unwrap stderr instead.
    // io::stderr().write_all(&output.stderr).unwrap();
});
