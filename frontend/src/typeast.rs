use crate::ast::{Expr, FnDecl, FnDef, StructDefType};

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
    pub body_def: bool,
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
            body_def: false,
        }
    }
}

impl From<FnDef> for FnType {
    fn from(fn_def: FnDef) -> Self {
        Self {
            params: fn_def.header.params.iter().map(|x| x.1.clone()).collect(),
            ret_type: Box::new(fn_def.header.ret_type.clone()),
            body_def: fn_def.body.is_some(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ArrayType {
    pub inner_type: Box<TypeDef>,
    pub index: Box<Expr>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TypeDef {
    Void,
    PrimType(PrimType),
    PointerType(Box<TypeDef>),
    Function(FnType),
    Alias(String),
    Struct(StructDefType),
    Array(ArrayType),
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
