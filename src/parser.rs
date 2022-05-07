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

        // if <expr> then <expr> else <expr>
        let if_expr = text::keyword("if")
            .ignore_then(expr.clone().map(Box::new))
            .then_ignore(text::keyword("then"))
            .then(expr.clone().map(Box::new))
            .then_ignore(text::keyword("else"))
            .then(expr.clone().map(Box::new))
            .map(|((test, then), r#else)| Expr::If(test, then, r#else));

        // while <expr> do <expr>
        let while_expr = text::keyword("while")
            .ignore_then(expr.clone().map(Box::new))
            .then_ignore(text::keyword("do"))
            .then(expr.clone().map(Box::new))
            .map(|(test, body)| Expr::While(test, body));

        choice((if_expr, while_expr, atom))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn int_lit(lit: &str) -> Expr {
        Expr::Lit(Lit::Int(String::from(lit)))
    }

    #[test]
    fn if_expr() {
        assert_eq!(
            expr().parse("if(0)then(1)else(2)"),
            Ok(Expr::If(
                Box::new(int_lit("0")),
                Box::new(int_lit("1")),
                Box::new(int_lit("2")),
            ))
        );
    }

    #[test]
    fn while_expr() {
        assert_eq!(
            expr().parse("while(0)do(1)"),
            Ok(Expr::While(Box::new(int_lit("0")), Box::new(int_lit("1"))))
        );
    }
}
