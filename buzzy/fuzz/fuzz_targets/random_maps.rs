#![no_main]
#![allow(unused_imports)]

use std::fs;
use std::fs::File;
use std::process::Command;
use std::io::{self, Write};

use chrono::{Utc, DateTime};

use arbitrary;
use libfuzzer_sys::fuzz_target;

extern crate buzzy;

use crate::buzzy::ebpf_generator::EbpfGenerator;
use crate::buzzy::random_maps_parser::RandomMapsParser;
use crate::buzzy::common::Map;

#[derive(arbitrary::Arbitrary, Debug)]
struct FuzzSeedData {
    maps: Vec<Map>,
}

fuzz_target!(|data: FuzzSeedData| {
    // Strategy and other settings
    let strategy = "RandomMaps";
    let maps_included = true;
    let verbose = false;

    // Generate eBPF program
    let mut generator = EbpfGenerator::new(strategy);
    generator.generate_program();
    let generated_program = generator.prog;

    // Pass it to the parser and parse it
    let parser = RandomMapsParser::new(generated_program, data.maps);
    let _parser_result = match parser.parse_prog() {
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
                 .args(&["--termination", "obj-files/data.o"])
                 .output()
                 .expect("failed to execute process");

    let str_v_output = String::from_utf8(verify_output.stdout).unwrap();
    
    // PREVAIL outputs 0 for invalid, and 1 for valid eBPF programs
    if str_v_output.starts_with("1") {
        // Execute the eBPF program with uBPF (-j flag for JIT compile)
        let execute_output = Command::new("../ubpf/vm/test")
                 .args(&["-j", "obj-files/data.o"])
                 .output()
                 .expect("failed to execute process");

        let str_e_output = String::from_utf8(execute_output.stdout).unwrap();

        // uBPF outputs PRIx64 string, encoding memory and memory length if the program was executed
        if str_e_output.starts_with("0x") {
            if verbose == true {
                println!("uBPF result: {}", str_e_output);
            }
            // TODO: Check for memory bugs if PREVAIL="1" and uBPF="0x ..."
        }
        else { // Hitting this branch should not happen; means that PREVAIL or uBPF has a bug/inconsistency
            if verbose == true {
                let str_e_error = String::from_utf8(execute_output.stderr).unwrap();
                println!("uBPF error: {}", str_e_error);
            }
            
            let now = Utc::now().timestamp_millis().to_string();
            let file_name = "logs/error".to_owned() + &now + ".o";
            let _file = File::create(file_name.clone());

            let _file_write_result = match fs::copy("obj-files/data.o", file_name) {
                Ok(_) => {
                    // Do nothing
                },
                Err(_) => {
                    return;
                }
            };
        }
    }
});
