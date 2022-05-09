use crate::ast::{Expr, Lit, Stat};
use chumsky::prelude::*;

pub fn lit() -> impl Parser<char, Lit, Error = Simple<char>> {
    text::digits(10).map(Lit::Int)
}

pub fn parsers() -> (
    impl Parser<char, Expr, Error = Simple<char>>,
    impl Parser<char, Stat, Error = Simple<char>>,
) {
    let mut expr = Recursive::declare();
    let mut stat = Recursive::declare();

    expr.define({
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

        // { <stat>* }
        let block_expr = stat
            .clone()
            .then_ignore(just(';'))
            .repeated()
            .map(Expr::Block)
            .delimited_by(just('{'), just('}'));

        choice((if_expr, while_expr, block_expr, atom))
    });

    stat.define({
        // var <ident> = <expr>
        let var_stat = text::keyword("var")
            .ignore_then(text::whitespace())
            .ignore_then(text::ident())
            .then_ignore(just('='))
            .then(expr.clone())
            .map(|(ident, body)| Stat::Var(ident, body));

        var_stat
    });

    (expr, stat)
}

pub fn expr() -> impl Parser<char, Expr, Error = Simple<char>> {
    parsers().0
}

pub fn stat() -> impl Parser<char, Stat, Error = Simple<char>> {
    parsers().1
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

    #[test]
    fn block_expr() {
        assert_eq!(
            expr().parse(r"{var hoge=0;}"),
            Ok(Expr::Block(vec![Stat::Var(
                String::from("hoge"),
                int_lit("0")
            )]))
        );
    }

    #[test]
    fn var_stat() {
        assert_eq!(
            stat().parse("var hoge=0"),
            Ok(Stat::Var(String::from("hoge"), int_lit("0")))
        );
    }
}
