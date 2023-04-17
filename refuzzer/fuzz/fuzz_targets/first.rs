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
    let strategy = "InitZero";
    let mut generator = EbpfGenerator::new(data.seed, strategy);
    let generated_program = generator.generate_program();

    // Pass it to the parser and parse it
    let parser = ElfParser::new(generated_program);
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
    let verify_output = Command::new("../ebpf-verifier/check")
                 .args(&["../obj-files/data.o"])
                 .output()
                 .expect("failed to execute process");

    let str_v_output = String::from_utf8(verify_output.stdout).unwrap();
    
    if str_v_output.starts_with("1") {
        // Execute the eBPF program with uBPF (-j flag for JIT compile)
        let execute_output = Command::new("../ubpf/vm/test")
                 .args(&["../obj-files/data.o"])
                 .output()
                 .expect("failed to execute process");
    }
    
    // If no errors occur when running, unwrap stdout.
    // io::stdout().write_all(&verify_output.stdout).unwrap();
    // io::stdout().write_all(&execute_output.stdout).unwrap();

    // If any errors occur when running, unwrap stderr instead.
    // io::stderr().write_all(&verify_output.stderr).unwrap();
    // io::stderr().write_all(&execute_output.stderr).unwrap();
});
