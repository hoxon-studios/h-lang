//use backends::x86_64::X86_64;

use crate::parser::Parser;

//mod backends;
mod parser;

fn main() {
    let mut code = String::new();
    loop {
        let mut buffer = String::new();
        let _ = std::io::stdin().read_line(&mut buffer);
        if buffer.starts_with(":quit") {
            break;
        } else if buffer.starts_with(":clear") {
            code.clear();
        } else if buffer.starts_with(":build") {
            println!("CODE:");
            println!("{}", code);
            let output = Parser::parse(&code);

            println!("-ASM------------------");
            println!("{output}");
            println!("----------------------");
        } else {
            code.push_str(&buffer);
        }
    }
}
