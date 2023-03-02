use crate::lexer::Operator;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Val {
    Integer(i64),
    Char(char),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Program {
    pub var_decls: Vec<VarDecl>,
    pub fn_defs: Vec<FnDef>,
    pub fn_decl: Vec<FnDecl>,
    pub main: Option<FnDef>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Expr {
    BinOp(Operator, Box<Expr>, Box<Expr>),
    UnaryPreOp(Operator, Box<Expr>),
    UnaryPostOp(Operator, Box<Expr>),
    Value(Val),
    Ident(String),
    Call(Box<Expr>, Vec<Expr>),
    Index(Box<Expr>, Box<Expr>),
    Deref(Box<Expr>),
    Address(Box<Expr>),
    Cast(TypeDef, Box<Expr>),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct VarDecl {
    pub name: String,
    pub var_type: TypeDef,
    pub init_val: Option<Expr>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Statement {
    Expr(Expr),
    VarDecl(VarDecl),
    Block(Vec<Statement>),
    If(Expr, Box<Statement>),
    IfElse(Expr, Box<Statement>, Box<Statement>),
    For(Option<Box<Statement>>, Option<Expr>, Option<Expr>, Box<Statement>),
    While(Expr, Box<Statement>),
    Break,
    Continue,
    Return(Option<Box<Expr>>),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum PrimType {
    Int,
    Char,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TypeDef {
    Void,
    PrimType(PrimType),
    PointerType(Box<TypeDef>),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FnDecl {
    pub name: String,
    pub params: Vec<(String, TypeDef)>,
    pub ret_type: TypeDef,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FnDef {
    pub header: FnDecl,
    pub body: Statement,
}
