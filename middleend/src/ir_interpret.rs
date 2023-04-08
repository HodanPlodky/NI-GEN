use std::collections::HashMap;

use crate::{
    inst::{BBIndex, BasicBlock, ImmI, InstructionType, Reg, Register},
    ir::{Function, IrProgram},
};

pub enum InterpretError {
    InvalidAddress(Value),
    OutOfBoundRead(Value),
    NonExistingRead(Register),
    DoubleWrite(Register),
    BasicBlockConti,
    Unknown,
}

pub fn run(program: IrProgram) -> Result<(), InterpretError> {
    todo!()
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Value {
    Unsigned(u64),
    Signed(i64),
    Char(u8),
}

type Env = HashMap<Register, Value>;

struct Interpret {
    stack_size: usize,
    stack: Vec<u8>,
    heap: Vec<u8>,
    globals: Env,
    locals: Vec<Env>,
    program: IrProgram,
}

impl Interpret {
    fn new(program: IrProgram, stack_size: usize) -> Self {
        Self {
            stack_size,
            stack: vec![],
            heap: vec![],
            globals: HashMap::new(),
            locals: vec![],
            program,
        }
    }

    fn set(&mut self, reg: Register, value: Value) -> Result<(), InterpretError> {
        if self.locals.is_empty() {
            if self.globals.contains_key(&reg) {
                return Err(InterpretError::DoubleWrite(reg));
            }
            self.globals.insert(reg, value);
        } else {
            if self.locals.last().unwrap().contains_key(&reg) {
                return Err(InterpretError::DoubleWrite(reg));
            }
            self.locals.last_mut().unwrap().insert(reg, value);
        }
        Ok(())
    }

    fn get(&mut self, reg: Register) -> Result<Value, InterpretError> {
        if let Some(locals) = self.locals.last() {
            if let Some(val) = locals.get(&reg) {
                return Ok(*val);
            }
        }
        if let Some(val) = self.globals.get(&reg) {
            return Ok(*val);
        }
        Err(InterpretError::NonExistingRead(reg))
    }

    fn run(&mut self) -> Result<(), InterpretError> {
        todo!()
    }

    fn run_func(&mut self, func: Function) -> Result<Value, InterpretError> {
        todo!()
    }

    fn mem_read(&self, val : Value) -> Result<Value, InterpretError> {
        todo!()
    }

    fn run_basicblock(&mut self, bb: &BasicBlock) -> Result<Option<BBIndex>, InterpretError> {
        let mut next = None;
        for inst in bb.instruction.iter() {
            if next.is_some() {
                return Err(InterpretError::BasicBlockConti);
            }
            match inst.data {
                InstructionType::Ldi(ImmI(imm)) => self.set(inst.id, Value::Signed(imm))?,
                InstructionType::Ld(Reg(reg)) => {
                    let val = self.get(reg)?;

                }
                InstructionType::St(_) => todo!(),
                InstructionType::Alloca(_) => todo!(),
                InstructionType::Allocg(_) => todo!(),
                InstructionType::Cpy(_) => todo!(),
                InstructionType::Gep(_) => todo!(),
                InstructionType::Add(_) => todo!(),
                InstructionType::Sub(_) => todo!(),
                InstructionType::Mul(_) => todo!(),
                InstructionType::Div(_) => todo!(),
                InstructionType::Mod(_) => todo!(),
                InstructionType::Shr(_) => todo!(),
                InstructionType::Shl(_) => todo!(),
                InstructionType::And(_) => todo!(),
                InstructionType::Or(_) => todo!(),
                InstructionType::Xor(_) => todo!(),
                InstructionType::Neg(_) => todo!(),
                InstructionType::Lt(_) => todo!(),
                InstructionType::Le(_) => todo!(),
                InstructionType::Gt(_) => todo!(),
                InstructionType::Ge(_) => todo!(),
                InstructionType::Eql(_) => todo!(),
                InstructionType::Fun(_) => todo!(),
                InstructionType::Call(_) => todo!(),
                InstructionType::Arg(_) => todo!(),
                InstructionType::Ret(_) => todo!(),
                InstructionType::Retr(_) => todo!(),
                InstructionType::Jmp(_) => todo!(),
                InstructionType::Branch(_) => todo!(),
            }
        }
        Ok(next)
    }
}
