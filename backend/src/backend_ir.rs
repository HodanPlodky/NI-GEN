use crate::insts::AsmInstruction;


type Data = Vec<u8>;

pub type AsmBasicBlock = Vec<AsmInstruction>;

pub struct AsmFunction {
    pub name: String,
    pub blocks: Vec<AsmBasicBlock>,
}

pub struct AsmProgram {
    pub data: Vec<(String, Data)>,
    pub start: AsmBasicBlock,
    pub text: Vec<AsmFunction>,
}

impl Default for AsmProgram {
    fn default() -> Self {
        Self {
            data: vec![],
            start: vec![],
            text: vec![],
        }
    }
}

