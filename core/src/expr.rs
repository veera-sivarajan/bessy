use crate::lexer::Token;

#[derive(Clone, Debug)]
pub enum Expr {
    Variable(VariableExpr),
    Binary(BinaryExpr),
    Unary(UnaryExpr),
    Number(NumberExpr),
    Boolean(bool),
    String(String),
    Assign(AssignExpr),
    Group(GroupExpr),
    Call(CallExpr),
    Nil,
}

#[derive(Clone, Debug)]
pub struct GroupExpr {
    pub expr: Box<Expr>,
}

#[derive(Clone, Debug)]
pub struct BinaryExpr {
    pub left: Box<Expr>,
    pub oper: Token,
    pub right: Box<Expr>,
}

#[derive(Clone, Debug)]
pub struct UnaryExpr {
    pub oper: Token,
    pub right: Box<Expr>,
}

#[derive(Clone, Debug)]
pub struct NumberExpr {
    pub value: f64,
}

#[derive(Clone, Debug)]
pub struct VariableExpr {
    pub name: Token,
}

#[derive(Clone, Debug)]
pub struct AssignExpr {
    pub name: Token,
    pub value: Box<Expr>,
}

#[derive(Clone, Debug)]
pub struct CallExpr {
    pub callee: Box<Expr>,
    pub paren: Token,
    pub args: Vec<Expr>,
}
