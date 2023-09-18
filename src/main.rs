use std::env;

use crate::assembler::Assembler;

mod parser;
mod code;
mod assembler;
mod symbol_table;

fn main() {
    match env::args().nth(1) {
        Some(input_file) => {
            println!("Start assembling for '{}", input_file);
            Assembler::new().run(&input_file);
            println!("Completed");
        },
        None => {
            println!("Usage: hack_assembler <input_file>");
        }
    }
}
