use crate::lexer::Operator;

pub enum Val {
    Integer(i64),
    Char(char),
}

pub struct Program {
    pub var_decls: Vec<VarDecl>,
    pub fn_defs: Vec<FnDef>,
    pub fn_decl: Vec<FnDecl>,
    pub main: Option<FnDef>,
}

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

pub struct VarDecl {
    pub name: String,
    pub var_type: TypeDef,
    pub init_val: Option<Expr>,
}

pub enum Statement {
    Expr(Expr),
    VarDecl(VarDecl),
    Block(Vec<Statement>),
    If(Expr, Box<Statement>),
    IfElse(Expr, Box<Statement>, Box<Statement>),
    For(Option<Box<Statement>>, Option<Expr>, Option<Expr>),
    While(Expr, Box<Statement>),
    Break,
    Continue,
    Return(Option<Box<Expr>>),
}

pub enum PrimType {
    Int,
    Char,
}

pub enum TypeDef {
    Void,
    PrimType(PrimType),
    PointerType(Box<TypeDef>),
}

pub struct FnDecl {
    pub name: String,
    pub params: Vec<(String, TypeDef)>,
    pub ret_type: TypeDef,
}

pub struct FnDef {
    pub header: FnDecl,
    pub body: Statement,
}
