use crate::ast::Lit;
use chumsky::prelude::*;

pub fn lit() -> impl Parser<char, Lit, Error = Simple<char>> {
    text::digits(10).map(Lit::Int)
}
