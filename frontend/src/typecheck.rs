use std::collections::HashMap;

use crate::ast::TypeDef;

pub struct Typechecker {
    type_map : HashMap<String, TypeDef>,
}

impl Default for Typechecker {
    fn default() -> Self {
        todo!()
    }
}

impl Typechecker {
    fn get_type(&self, symbol : String) -> Option<TypeDef> {
        self.type_map.get(&symbol).cloned()
    }

    fn type_ast(&self) {
        todo!()
    }
}
