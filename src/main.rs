use std::process::exit;

use shell::Shell;

// main.rs
//
mod assembler;
mod constant;
mod cpu;
mod memory;
mod opcode;
mod shell;
mod vio;
fn main() {
    let mut shell = match Shell::new() {
        Ok(shell) => shell,
        Err(why) => {
            println!("failed to initalize shell instance :: {}", why);
            exit(0)
        }
    };
    shell.start();
}
