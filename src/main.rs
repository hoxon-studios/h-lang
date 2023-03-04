//use backends::x86_64::X86_64;

use crate::parser::Parser;

//mod backends;
mod parser;

fn main() {
    //let mut context = X86_64::init();
    loop {
        let mut buffer = String::new();
        let _ = std::io::stdin().read_line(&mut buffer);
        if buffer.starts_with(":q") {
            break;
        } else if buffer.starts_with(":clear") {
        } else {
            let output = Parser::parse(&buffer);

            println!("-ASM------------------");
            println!("{output}");
            println!("----------------------");
        }
    }
}
