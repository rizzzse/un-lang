mod ast;
mod parser;

use chumsky::prelude::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let code = args[1..].join(" ");
    println!("input: \"{}\"", code);
    println!("ast: {:?}", parser::lit().parse(code));
}
