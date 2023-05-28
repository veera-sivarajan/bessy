use crate::expr::Expr;
use crate::lexer::Token;
#[derive(Clone, Debug)]
pub enum Stmt {
    Var {
        name: Token,
        init: Option<Expr>,
    },
    Print(Expr),
    Expression(Expr),
    Block(Vec<Stmt>),
    If {
        condition: Expr,
        then: Box<Stmt>,
        elze: Option<Box<Stmt>>,
    },
    While {
        condition: Expr,
        body: Box<Stmt>,
    },
    Function {
        name: Token,
        params: Vec<Token>,
        body: Vec<Stmt>,
    },
    Return {
        keyword: Token,
        value: Option<Expr>,
    }
}

