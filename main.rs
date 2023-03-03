use std::process::Command;
use std::io::{self, Write};
//use std::env;

fn main() {
    // Backtrace environment variable for debugging.
    //env::set_var("RUST_BACKTRACE", "1");
    println!("Hello, world!");
    let output = Command::new("../ebpf-verifier/check")
                .args(&["../ebpf-verifier/ebpf-samples/cilium/bpf_lxc.o","2/1"])
                .output()
                .expect("failed to execute process");
    
    //println!("output: {}", output.status);
    // If no errors occur when running, unwrap stdout.
    io::stdout().write_all(&output.stdout).unwrap();
    // If any errors occur when running, unwrap stderr instead.
    //io::stderr().write_all(&output.stderr).unwrap();
}
