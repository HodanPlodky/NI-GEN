use crate::lexer::Operator;

pub enum Val {
    Integer(i64),
    Char(char),
}

pub struct Program {
    var_decls: Vec<VarDecl>,
    fn_defs: Vec<FnDef>,
    fn_decl: Vec<FnDecl>,
    main: FnDef,
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
    name: String,
    params: Vec<(String, TypeDef)>,
    ret_type: TypeDef,
}

pub struct FnDef {
    header: FnDecl,
    body: Statement,
}
