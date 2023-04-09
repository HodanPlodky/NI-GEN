use std::collections::HashMap;

use crate::{
    inst::{
        BBIndex, BasicBlock, ImmC, ImmI, Instruction, InstructionType, Reg, RegReg, RegType,
        Register,
    },
    ir::{Function, IrProgram},
};

pub enum InterpretError {
    VoidRegister(Instruction),
    InvalidAddress(Value),
    OutOfBoundRead(Value),
    OutOfBoundWrite,
    NonExistingRead(Register),
    DoubleWrite(Register),
    InvalidOp(Instruction),
    BasicBlockConti,
    Unknown,
}

pub fn run(program: IrProgram) -> Result<(), InterpretError> {
    todo!()
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Value {
    Signed(i64),
    Char(u8),
}

type Addr = usize;

impl From<Value> for Addr {
    fn from(value: Value) -> Self {
        match value {
            Value::Signed(x) => x as usize,
            Value::Char(x) => x as usize,
        }
    }
}

type Env = HashMap<Register, Value>;

struct Memory {
    stack_size: usize,
    stack: Vec<u8>,
    heap: Vec<u8>,
}

impl Memory {
    fn new(stack_size: usize) -> Self {
        Self {
            stack_size,
            stack: vec![],
            heap: vec![],
        }
    }

    fn read_int(&self, addr_val: Value) -> Result<Value, InterpretError> {
        let mut addr: usize = addr_val.into();

        let mut res: i64 = 0;
        let vec = if addr < self.stack_size {
            &self.stack
        } else {
            addr -= self.stack_size;
            &self.heap
        };

        for i in (0..8).rev() {
            res <<= 8;
            res |= if let Some(x) = vec.get(addr + i) {
                x.clone() as i64
            } else {
                return Err(InterpretError::OutOfBoundRead(addr_val));
            }
        }
        Ok(Value::Signed(res))
    }

    fn read_char(&self, addr_val: Value) -> Result<Value, InterpretError> {
        let mut addr: usize = addr_val.into();

        let vec = if addr < self.stack_size {
            &self.stack
        } else {
            addr -= self.stack_size;
            &self.heap
        };

        if let Some(val) = vec.get(addr) {
            Ok(Value::Char(*val))
        } else {
            Err(InterpretError::OutOfBoundRead(addr_val))
        }
    }

    fn write(&mut self, addr: Value, value: Value) -> Result<(), InterpretError> {
        todo!()
    }

    fn alloca(&mut self, amount: i64) -> Result<Value, InterpretError> {
        if self.stack.len() + amount as usize > self.stack_size {
            return Err(InterpretError::OutOfBoundWrite);
        }
        let res = self.stack.len();
        for i in 0..amount {
            self.stack.push(0);
        }
        Ok(Value::Signed(res as i64))
    }
}

struct Interpret {
    mem: Memory,
    globals: Env,
    locals: Vec<Env>,
    program: IrProgram,
}

impl Interpret {
    fn new(program: IrProgram, stack_size: usize) -> Self {
        Self {
            mem: Memory::new(stack_size),
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

    fn binary_op(
        &mut self,
        inst: Instruction,
        regs: RegReg,
        op_i64: &dyn Fn(i64, i64) -> i64,
        op_u8: &dyn Fn(u8, u8) -> u8,
    ) -> Result<(), InterpretError> {
        let (l, r) = (regs.0, regs.1);
        let l_val = self.get(l)?;
        let r_val = self.get(r)?;
        let val = match (l_val, r_val) {
            (Value::Signed(a), Value::Signed(b)) => Ok(Value::Signed(op_i64(a, b))),
            (Value::Char(a), Value::Char(b)) => Ok(Value::Char(op_u8(a, b))),
            _ => Err(InterpretError::InvalidOp(inst.clone())),
        }?;
        self.set(inst.id, val)?;
        Ok(())
    }

    fn logic_bin_op(
        &mut self,
        inst: Instruction,
        regs: RegReg,
        op_i64: &dyn Fn(i64, i64) -> bool,
        op_u8: &dyn Fn(u8, u8) -> bool,
    ) -> Result<(), InterpretError> {
        let (l, r) = (regs.0, regs.1);
        let l_val = self.get(l)?;
        let r_val = self.get(r)?;
        let val = match (l_val, r_val) {
            (Value::Signed(a), Value::Signed(b)) => {
                Ok(Value::Signed(if op_i64(a, b) { 1 } else { 0 }))
            }
            (Value::Char(a), Value::Char(b)) => Ok(Value::Signed(if op_u8(a, b) { 1 } else { 0 })),
            _ => Err(InterpretError::InvalidOp(inst.clone())),
        }?;
        self.set(inst.id, val)?;
        Ok(())
    }

    fn run_basicblock(&mut self, bb: &BasicBlock) -> Result<Option<BBIndex>, InterpretError> {
        let mut next = None;
        for inst in bb.instruction.iter() {
            if next.is_some() {
                return Err(InterpretError::BasicBlockConti);
            }
            match inst.data {
                InstructionType::Ldi(ImmI(imm)) => self.set(inst.id, Value::Signed(imm))?,
                InstructionType::Ldc(ImmC(imm)) => self.set(inst.id, Value::Char(imm as u8))?,
                InstructionType::Ld(Reg(reg)) => {
                    let val = self.get(reg)?;
                    let val = match inst.reg_type {
                        RegType::Void => Err(InterpretError::VoidRegister(inst.clone())),
                        RegType::Int => self.mem.read_int(val),
                        RegType::Char => self.mem.read_char(val),
                    }?;
                    self.set(inst.id, val)?
                }
                InstructionType::St(RegReg(reg_addr, reg_source)) => {
                    let addr_val = self.get(reg_addr)?;
                    let value = self.get(reg_source)?;
                    self.mem.write(addr_val, value)?;
                }
                InstructionType::Alloca(ImmI(imm)) | InstructionType::Allocg(ImmI(imm)) => {
                    let addr = self.mem.alloca(imm)?;
                    self.set(inst.id, addr)?
                }
                InstructionType::Cpy(_) => todo!(),
                InstructionType::Gep(_) => todo!(),
                InstructionType::Add(regs) => {
                    self.binary_op(inst.clone(), regs, &|a, b| a + b, &|a, b| a + b)?
                }
                InstructionType::Sub(regs) => {
                    self.binary_op(inst.clone(), regs, &|a, b| a - b, &|a, b| a - b)?
                }
                InstructionType::Mul(regs) => {
                    self.binary_op(inst.clone(), regs, &|a, b| a * b, &|a, b| a * b)?
                }
                InstructionType::Div(regs) => {
                    self.binary_op(inst.clone(), regs, &|a, b| a / b, &|a, b| a / b)?
                }
                InstructionType::Mod(regs) => {
                    self.binary_op(inst.clone(), regs, &|a, b| a % b, &|a, b| a % b)?
                }
                InstructionType::Shr(regs) => {
                    self.binary_op(inst.clone(), regs, &|a, b| a >> b, &|a, b| a >> b)?
                }
                InstructionType::Shl(regs) => {
                    self.binary_op(inst.clone(), regs, &|a, b| a << b, &|a, b| a << b)?
                }
                InstructionType::And(regs) => {
                    self.binary_op(inst.clone(), regs, &|a, b| a & b, &|a, b| a & b)?
                }
                InstructionType::Or(regs) => {
                    self.binary_op(inst.clone(), regs, &|a, b| a | b, &|a, b| a | b)?
                }
                InstructionType::Xor(regs) => {
                    self.binary_op(inst.clone(), regs, &|a, b| a ^ b, &|a, b| a ^ b)?
                }
                InstructionType::Neg(Reg(reg)) => {
                    let val = self.get(reg)?;
                    let val = match val {
                        Value::Signed(x) => Value::Signed(!x),
                        Value::Char(x) => Value::Char(!x),
                    };
                    self.set(inst.id, val)?;
                }
                InstructionType::Lt(regs) => {
                    self.logic_bin_op(inst.clone(), regs, &|a, b| a < b, &|a, b| a < b)?
                }
                InstructionType::Le(regs) => {
                    self.logic_bin_op(inst.clone(), regs, &|a, b| a <= b, &|a, b| a <= b)?
                }
                InstructionType::Gt(regs) => {
                    self.logic_bin_op(inst.clone(), regs, &|a, b| a > b, &|a, b| a > b)?
                }
                InstructionType::Ge(regs) => {
                    self.logic_bin_op(inst.clone(), regs, &|a, b| a >= b, &|a, b| a >= b)?
                }
                InstructionType::Eql(regs) => {
                    self.logic_bin_op(inst.clone(), regs, &|a, b| a == b, &|a, b| a == b)?
                }
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
