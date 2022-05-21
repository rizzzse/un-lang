mod ast;
mod gen;
mod parser;

use chumsky::prelude::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let code = args[1..].join(" ");
    println!("input: \"{}\"", code);
    let ast = parser::stat().parse(code);
    println!("ast: {:?}", ast);
    if let Ok(ast) = ast {
        println!("llvm-ir: {:?}", gen::gen_ir(ast));
    }
}
