use crate::lexer::Token;

#[derive(Clone, Debug)]
pub enum Expr {
    Variable(Token),
    Binary {
        left: Box<Expr>,
        oper: Token,
        right: Box<Expr>,
    },
    Unary {
        oper: Token,
        right: Box<Expr>,
    },
    Number(f64),
    Boolean(bool),
    String(String),
    Assign {
        name: Token,
        value: Box<Expr>,
    },
    Group {
        expr: Box<Expr>,
    },
    Call {
        callee: Box<Expr>,
        paren: Token,
        args: Vec<Expr>,
    },
    Nil,
}
