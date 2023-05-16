#![no_main]
#![allow(unused_imports)]

use std::fs;
use std::fs::{File, OpenOptions};
use std::process::Command;
use std::io::{self, Write, prelude::*};

use chrono::{Utc, DateTime};

use arbitrary;
use libfuzzer_sys::fuzz_target;

extern crate buzzy;
use crate::buzzy::random_bytes_parser::RandomBytesParser;
use crate::buzzy::common::Instruction;


#[derive(arbitrary::Arbitrary, Debug)]
struct FuzzData {
    pub prog: Vec<Instruction>,
}

fuzz_target!(|data: FuzzData| {
    let verbose = false;
    let prog_length = data.prog.len();

    // No program generation - the "eBPF program" is just random bytes, generated using Arbitrary

    let parser = RandomBytesParser::new(data.prog);
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
                 .args(&["obj-files/data.o"])
                 .output()
                 .expect("failed to execute process");

    let str_v_output = String::from_utf8(verify_output.stdout).unwrap();

    /***** COLLECT DATA FROM EXPERIMENTS *****/

    if str_v_output.len() == 0 {
        return;
    }

    // Remove '\n' for nice result format
    let mut no_new_line = &str_v_output[0..str_v_output.len()-1];
    if no_new_line.starts_with("unmarshaling error") {
        no_new_line = &str_v_output[0..no_new_line.len()-1];
    }

    // Append number of instructions to the result of PREVAIL
    let result = format!("{}", no_new_line);

    let mut exp_file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("logs/exp-data.txt")
        .unwrap();

    if let Err(e) = writeln!(exp_file, "{},{}", result, prog_length) {
        eprintln!("Couldn't write to file: {}", e);
    }

    // Checking if PREVAIL result is untouched - it currently is :)
    // println!("{}", str_v_output);

    /************************************************/
    
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
