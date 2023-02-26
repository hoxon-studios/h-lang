use backends::x86_64::X86_64;
use parser::parse;
use tokenizer::tokenize;

mod backends;
mod parser;
mod tokenizer;

fn main() {
    let mut context = X86_64::init();
    loop {
        let mut buffer = String::new();
        let _ = std::io::stdin().read_line(&mut buffer);
        if buffer.starts_with(":q") {
            break;
        } else if buffer.starts_with(":clear") {
            context = X86_64::init();
        } else {
            let tokens = tokenize(&buffer).unwrap();
            let expression = parse(tokens).unwrap();
            let output = context.compile(&expression);

            println!("-CONTEXT--------------");
            println!("{:#?}", context);
            println!("-ASM------------------");
            println!("{output}");
            println!("----------------------");
        }
    }
}
