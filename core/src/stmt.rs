use crate::expr::Expr;
use crate::lexer::Token;
#[derive(Clone, Debug)]
pub enum Stmt {
    Var(VarStmt),
    Print(PrintStmt),
    Expression(ExpressionStmt),
    Block(BlockStmt),
    If(IfStmt),
    While(WhileStmt),
    Function(FunStmt),
    Return(ReturnStmt),
}

#[derive(Clone, Debug)]
pub struct IfStmt {
    pub condition: Expr,
    pub then_branch: Box<Stmt>,
    pub else_branch: Option<Box<Stmt>>,
}

#[derive(Clone, Debug)]
pub struct WhileStmt {
    pub condition: Expr,
    pub body: Box<Stmt>,
}

#[derive(Clone, Debug)]
pub struct VarStmt {
    pub name: Token,
    pub init: Option<Expr>,
}

#[derive(Clone, Debug)]
pub struct PrintStmt {
    pub expr: Expr,
}

#[derive(Clone, Debug)]
pub struct ExpressionStmt {
    pub expr: Expr,
}

#[derive(Clone, Debug)]
pub struct BlockStmt {
    pub statements: Vec<Stmt>,
}

#[derive(Clone, Debug)]
pub struct FunStmt {
    pub name: Token,
    pub params: Vec<Token>,
    pub body: Vec<Stmt>,
}

#[derive(Clone, Debug)]
pub struct ReturnStmt {
    pub keyword: Token,
    pub value: Option<Expr>,
}
