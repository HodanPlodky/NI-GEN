use std::{collections::HashMap, fmt::Display};

use crate::{
    inst::{
        ImmC, ImmI, InstructionType, Reg, RegReg, SymRegs, TerminatorBranch, TerminatorJump,
        TerminatorReg,
    },
    ir::{BBIndex, BasicBlock, Function, Instruction, IrProgram, RegType, Register, Symbol, InstUUID},
};

#[derive(Debug)]
pub enum InterpretError {
    VoidRegister(InstUUID),
    InvalidAddress(Value),
    OutOfBoundRead(Value),
    InvalidCond(Value),
    OutOfBoundWrite,
    NonExistingRead(Register),
    DoubleWrite(Register),
    InvalidOp(Instruction),
    NoMain,
    WrongMainReturn,
    BasicBlockConti,
    Unknown,
}

pub fn run(program: IrProgram) -> Result<i64, InterpretError> {
    let mut inter = Interpret::new(program, 4096 * 4096);
    inter.run()
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Value {
    Signed(i64),
    Char(u8),
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Signed(val) => write!(f, "{}", val),
            Value::Char(val) => write!(f, "{}", val.clone() as char),
        }
    }
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
type Args = Vec<Value>;

struct Memory {
    stack_size: usize,
    stack: Vec<u8>,
    sp: usize,
    heap: Vec<u8>,
}

impl Memory {
    fn new(stack_size: usize) -> Self {
        let mut stack = vec![];
        let mut heap = vec![];
        stack.resize(stack_size, 0);
        heap.resize(stack_size * 2, 0);
        Self {
            stack_size,
            sp: 0,
            stack,
            heap,
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
        let addr: usize = addr.into();

        match value {
            Value::Signed(v) => self.write_int(addr, v),
            Value::Char(c) => self.write_char(addr, c),
        }
    }

    fn write_int(&mut self, addr: usize, value: i64) -> Result<(), InterpretError> {
        let mut addr = addr;
        let vec = if addr < self.stack_size {
            &mut self.stack
        } else {
            addr -= self.stack_size;
            &mut self.heap
        };

        let mut value: u64 = value as u64;

        for i in 0..8 {
            vec[addr + i] = (0xff & value) as u8;
            value >>= 8;
        }
        Ok(())
    }

    fn write_char(&mut self, addr: usize, value: u8) -> Result<(), InterpretError> {
        let mut addr = addr;
        let vec = if addr < self.stack_size {
            &mut self.stack
        } else {
            addr -= self.stack_size;
            &mut self.heap
        };

        vec[addr] = value;
        Ok(())
    }

    fn alloca(&mut self, amount: i64) -> Result<Value, InterpretError> {
        if self.sp + amount as usize > self.stack_size {
            return Err(InterpretError::OutOfBoundWrite);
        }
        let res = self.stack.len();
        for _ in 0..amount {
            self.stack.push(0);
        }
        Ok(Value::Signed(res as i64))
    }
}

struct Interpret {
    mem: Memory,
    globals: Env,
    locals: Vec<Env>,
    args: Vec<Args>,
    program: IrProgram,
    rev_val: Option<Value>,
}

impl Interpret {
    fn new(program: IrProgram, stack_size: usize) -> Self {
        Self {
            mem: Memory::new(stack_size),
            globals: HashMap::new(),
            locals: vec![],
            args: vec![],
            program,
            rev_val: None,
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

    fn run(&mut self) -> Result<i64, InterpretError> {
        let glob_block = self.program.glob.clone();
        self.run_func(glob_block, vec![])?;
        let main = match self.program.funcs.get(&"main".to_string()) {
            Some(x) => Ok(x.clone()),
            None => Err(InterpretError::NoMain),
        }?;
        //self.locals.push(HashMap::new());
        self.run_func(main.clone(), vec![])?;
        match self.rev_val {
            Some(Value::Signed(val)) => Ok(val),
            None => Ok(0),
            _ => Err(InterpretError::WrongMainReturn),
        }
    }

    fn run_func(
        &mut self,
        func: Function,
        args: Vec<Value>,
    ) -> Result<Option<Value>, InterpretError> {
        self.locals.push(HashMap::new());
        self.args.push(args);
        let mut act = func.start();
        loop {
            let index = self.run_basicblock(act)?;
            if let Some(i) = index {
                act = &func.get_bb(i);
            } else {
                break;
            }
        }
        self.args.pop();
        self.locals.pop();
        Ok(self.rev_val)
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
        let mut terminated = false;
        for inst in bb.iter() {
            let inst = inst.clone();
            if terminated {
                return Err(InterpretError::BasicBlockConti);
            }
            match &inst.data {
                InstructionType::Ldi(ImmI(imm)) => self.set(inst.id, Value::Signed(*imm))?,
                InstructionType::Ldc(ImmC(imm)) => self.set(inst.id, Value::Char(*imm as u8))?,
                InstructionType::Ld(Reg(reg)) => {
                    let val = self.get(*reg)?;
                    let val = match inst.reg_type {
                        RegType::Void => Err(InterpretError::VoidRegister(inst.id)),
                        RegType::Int => self.mem.read_int(val),
                        RegType::Char => self.mem.read_char(val),
                    }?;
                    self.set(inst.id, val)?
                }
                InstructionType::St(RegReg(reg_addr, reg_source)) => {
                    let addr_val = self.get(*reg_addr)?;
                    let value = self.get(*reg_source)?;
                    self.mem.write(addr_val, value)?;
                }
                InstructionType::Alloca(ImmI(imm)) | InstructionType::Allocg(ImmI(imm)) => {
                    let addr = self.mem.alloca(*imm)?;
                    self.set(inst.id, addr)?
                }
                InstructionType::Cpy(_) => todo!(),
                InstructionType::Gep(_) => todo!(),
                InstructionType::Add(regs) => {
                    self.binary_op(inst.clone(), *regs, &|a, b| a + b, &|a, b| a + b)?
                }
                InstructionType::Sub(regs) => {
                    self.binary_op(inst.clone(), *regs, &|a, b| a - b, &|a, b| a - b)?
                }
                InstructionType::Mul(regs) => {
                    self.binary_op(inst.clone(), *regs, &|a, b| a * b, &|a, b| a * b)?
                }
                InstructionType::Div(regs) => {
                    self.binary_op(inst.clone(), *regs, &|a, b| a / b, &|a, b| a / b)?
                }
                InstructionType::Mod(regs) => {
                    self.binary_op(inst.clone(), *regs, &|a, b| a % b, &|a, b| a % b)?
                }
                InstructionType::Shr(regs) => {
                    self.binary_op(inst.clone(), *regs, &|a, b| a >> b, &|a, b| a >> b)?
                }
                InstructionType::Shl(regs) => {
                    self.binary_op(inst.clone(), *regs, &|a, b| a << b, &|a, b| a << b)?
                }
                InstructionType::And(regs) => {
                    self.binary_op(inst.clone(), *regs, &|a, b| a & b, &|a, b| a & b)?
                }
                InstructionType::Or(regs) => {
                    self.binary_op(inst.clone(), *regs, &|a, b| a | b, &|a, b| a | b)?
                }
                InstructionType::Xor(regs) => {
                    self.binary_op(inst.clone(), *regs, &|a, b| a ^ b, &|a, b| a ^ b)?
                }
                InstructionType::Neg(Reg(reg)) => {
                    let val = self.get(*reg)?;
                    let val = match val {
                        Value::Signed(x) => Value::Signed(!x),
                        Value::Char(x) => Value::Char(!x),
                    };
                    self.set(inst.id, val)?;
                }
                InstructionType::Lt(regs) => {
                    self.logic_bin_op(inst.clone(), *regs, &|a, b| a < b, &|a, b| a < b)?
                }
                InstructionType::Le(regs) => {
                    self.logic_bin_op(inst.clone(), *regs, &|a, b| a <= b, &|a, b| a <= b)?
                }
                InstructionType::Gt(regs) => {
                    self.logic_bin_op(inst.clone(), *regs, &|a, b| a > b, &|a, b| a > b)?
                }
                InstructionType::Ge(regs) => {
                    self.logic_bin_op(inst.clone(), *regs, &|a, b| a >= b, &|a, b| a >= b)?
                }
                InstructionType::Eql(regs) => {
                    self.logic_bin_op(inst.clone(), *regs, &|a, b| a == b, &|a, b| a == b)?
                }
                InstructionType::Call(_) => todo!(),
                InstructionType::CallDirect(SymRegs(sym, regs)) => {
                    let mut vals: Vec<Value> = vec![];
                    for reg in regs {
                        vals.push(self.get(*reg)?);
                    }
                    let Symbol(name) = sym;
                    let func = self.program.funcs.get(name).unwrap();
                    let res = self.run_func(func.clone(), vals)?;
                    match res {
                        Some(value) => self.set(inst.id, value)?,
                        None => (),
                    };
                }
                InstructionType::Arg(ImmI(index)) => {
                    let val = self.args.last().unwrap()[*index as usize];
                    self.set(inst.id, val)?;
                }
                InstructionType::Ret(_) | InstructionType::Exit(_) => {
                    terminated = true;
                    next = None;
                    self.rev_val = None;
                }
                InstructionType::Retr(TerminatorReg(reg)) => {
                    terminated = true;
                    next = None;
                    self.rev_val = Some(self.get(*reg)?);
                }
                InstructionType::Jmp(TerminatorJump(n)) => {
                    terminated = true;
                    next = Some(n)
                }
                InstructionType::Branch(TerminatorBranch(reg, ok, fail)) => {
                    let val = match self.get(*reg)? {
                        Value::Signed(val) => Ok(if val == 0 { false } else { true }),
                        v => Err(InterpretError::InvalidCond(v)),
                    }?;
                    terminated = true;
                    next = if val { Some(ok) } else { Some(fail) }
                }
                InstructionType::Print(Reg(reg)) => {
                    let val = self.get(*reg)?;
                    print!("{}", val);
                }
                InstructionType::Phi(_) => todo!(),
            }
        }
        Ok(next.copied())
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        builder::IrBuilder,
        inst::Terminator,
    };

    use super::*;

    type I = InstructionType;

    #[test]
    fn basic_interpret_test() {
        let context = crate::builder::BuildContext::default();
        let mut builder = IrBuilder::new(&context);
        builder.add(I::Ret(Terminator), RegType::Void);
        let mut fn_b = builder.create_fnbuild(0, RegType::Void);
        let reg = fn_b.add(I::Ldi(ImmI(5)), RegType::Int);
        fn_b.add(I::Print(Reg(reg)), RegType::Void);
        fn_b.add(I::Ret(Terminator), RegType::Void);
        builder.add_fn(fn_b.create("main")).unwrap();

        let mut inter = Interpret::new(builder.create(), 1024);
        inter.run().unwrap();
    }
}
