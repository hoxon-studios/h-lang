//use backends::x86_64::X86_64;

use std::{
    fs::{self, OpenOptions},
    io::Write,
};

use crate::parser::Parser;

//mod backends;
mod parser;

fn main() {
    let buffer = fs::read_to_string("L0/main.hx").expect("Failed to read file");
    let output = Parser::parse(&buffer);
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("L1/main.asm")
        .expect("Unable to create output file");
    file.write_all(output.as_bytes())
        .expect("Failed to write output file");
}
