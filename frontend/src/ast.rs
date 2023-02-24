pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

pub enum Val {
    Integer(i32),
    Char(char),
}

pub struct Program {}

pub enum Expr {
    BinOp(Op, Box<Expr>, Box<Expr>),
    Value(Val),
    Call(String, Vec<Expr>),
    Deref()
}

pub enum Statement {
    VarDecl(String),
    Block(Vec<Statement>),
    If(Expr, Box<Statement>),
    IfElse(Expr, Box<Statement>, Box<Statement>),
}

pub enum PrimType {
    Int,
    Char,
}

pub enum TypeDef {
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
