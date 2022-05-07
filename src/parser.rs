use crate::ast::{Expr, Lit};
use chumsky::prelude::*;

pub fn lit() -> impl Parser<char, Lit, Error = Simple<char>> {
    text::digits(10).map(Lit::Int)
}

pub fn expr() -> impl Parser<char, Expr, Error = Simple<char>> {
    recursive(|expr| {
        let atom = choice((
            lit().map(Expr::Lit),
            expr.clone().delimited_by(just('('), just(')')),
        ))
        .boxed();

        atom
    })
}
