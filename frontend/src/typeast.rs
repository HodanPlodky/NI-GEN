use crate::ast::{FnDecl, FnDef, StructDefType};

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
    pub index: usize,
}

#[derive(Debug, Eq, Clone)]
pub enum TypeDef {
    Void,
    PrimType(PrimType),
    PointerType(Box<TypeDef>),
    Function(FnType),
    Alias(String),
    Struct(StructDefType),
    Array(ArrayType),
}

impl PartialEq for TypeDef {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (TypeDef::Void, TypeDef::Void) => true,
            (TypeDef::PrimType(prim_a), TypeDef::PrimType(prim_b)) => prim_a == prim_b,
            (TypeDef::PointerType(inner_a), TypeDef::PointerType(inner_b)) => inner_a == inner_b,
            (TypeDef::PointerType(inner_ptr), TypeDef::Array(ArrayType { inner_type, index : _ })) => inner_ptr == inner_type,
            (TypeDef::Function(fn_type_a), TypeDef::Function(fn_type_b)) => fn_type_a == fn_type_b,
            (TypeDef::Alias(name_a), TypeDef::Alias(name_b)) => name_a == name_b,
            (TypeDef::Struct(struct_a), TypeDef::Struct(struct_b)) => struct_a == struct_b,
            (TypeDef::Array(ArrayType { inner_type, index : _ }), TypeDef::PointerType(inner_ptr)) => inner_type == inner_ptr,
            (TypeDef::Array(arr_a), TypeDef::Array(arr_b)) => arr_a == arr_b,
            _ => false
        }
    }
}

impl TypeDef {
    pub fn is_pointer(&self) -> bool {
        match self {
            TypeDef::PointerType(_) | TypeDef::Array(_) => true,
            _ => false,
        }
    }
}
