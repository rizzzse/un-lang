#[derive(Debug)]
pub enum Lit {
    Int(String),
}

#[derive(Debug)]
pub enum Expr {
    Lit(Lit),
}
