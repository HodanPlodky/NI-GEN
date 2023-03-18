use std::ops::{Deref, DerefMut};

use crate::lexer::{Loc, Operator};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct AstData {
    loc: Loc,
    pub node_type: Option<TypeDef>,
}

impl AstData {
    pub fn new(loc: Loc) -> Self {
        Self {
            loc,
            node_type: None,
        }
    }

    pub fn set_type(&mut self, t: TypeDef) -> TypeDef {
        self.node_type = Some(t.clone());
        t
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct AstNode<T>
where
    T: PartialEq + Eq + Clone,
{
    pub value: T,
    pub data: AstData,
}

impl<T> AstNode<T>
where
    T: PartialEq + Eq + Clone,
{
    pub fn new(value: T, data: AstData) -> Self {
        Self { value, data }
    }

    pub fn set_type(&mut self, t: TypeDef) -> TypeDef {
        self.data.set_type(t)
    }

    pub fn loc(&self) -> Loc {
        self.data.loc
    }

    pub fn typed(&self, t: TypeDef) -> Self {
        let mut data = self.data.clone();
        data.set_type(t);
        Self {
            value: self.value.clone(),
            data,
        }
    }

    // helper function for instances when I already know the type
    pub fn get_type(&self) -> TypeDef {
        self.data.node_type.clone().unwrap()
    }
}

impl<T> Deref for AstNode<T>
where
    T: PartialEq + Eq + Clone,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T> DerefMut for AstNode<T>
where
    T: PartialEq + Eq + Clone,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

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

impl Default for Program {
    fn default() -> Self {
        Self {
            var_decls: vec![],
            fn_defs: vec![],
            fn_decl: vec![],
            main: None,
        }
    }
}

pub type Expr = AstNode<ExprType>;

impl Into<Statement> for Expr {
    fn into(self) -> Statement {
        let data = self.data.clone();
        Statement::new(StatementType::Expr(self), data)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ExprType {
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

pub type VarDecl = AstNode<VarDeclType>;

impl Into<Statement> for VarDecl {
    fn into(self) -> Statement {
        let data = self.data.clone();
        Statement::new(StatementType::VarDecl(self), data)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct VarDeclType {
    pub name: String,
    pub var_type: TypeDef,
    pub init_val: Option<Expr>,
}

pub type Statement = AstNode<StatementType>;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum StatementType {
    Expr(Expr),
    VarDecl(VarDecl),
    Block(Vec<Statement>),
    If(Expr, Box<Statement>),
    IfElse(Expr, Box<Statement>, Box<Statement>),
    For(
        Option<Box<Statement>>,
        Option<Expr>,
        Option<Expr>,
        Box<Statement>,
    ),
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

impl From<PrimType> for TypeDef {
    fn from(p: PrimType) -> Self {
        TypeDef::PrimType(p)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FnType {
    pub params: Vec<TypeDef>,
    pub ret_type: Box<TypeDef>,
}

impl From<FnType> for TypeDef {
    fn from(f: FnType) -> Self {
        TypeDef::Function(f)
    }
}

impl From<FnDecl> for FnType {
    fn from(decl: FnDecl) -> Self {
        Self {
            params: decl.params.iter().map(|x| x.1.clone()).collect(),
            ret_type: Box::new(decl.ret_type.clone()),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TypeDef {
    Void,
    PrimType(PrimType),
    PointerType(Box<TypeDef>),
    Function(FnType),
}

impl TypeDef {
    pub fn is_pointer(&self) -> bool {
        if let TypeDef::PointerType(_) = self {
            true
        } else {
            false
        }
    }
}

pub type FnDecl = AstNode<FnDeclType>;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FnDeclType {
    pub name: String,
    pub params: Vec<(String, TypeDef)>,
    pub ret_type: TypeDef,
}

pub type FnDef = AstNode<FnDefType>;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FnDefType {
    pub header: FnDecl,
    pub body: Statement,
}
