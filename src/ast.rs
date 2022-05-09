#[derive(PartialEq, Debug)]
pub enum Lit {
    Int(String),
}

#[derive(PartialEq, Debug)]
pub enum Expr {
    Lit(Lit),
    If(Box<Self>, Box<Self>, Box<Self>),
    While(Box<Self>, Box<Self>),
    Block(Vec<Stat>),
}

#[derive(PartialEq, Debug)]
pub enum Stat {
    Var(String, Expr),
}
