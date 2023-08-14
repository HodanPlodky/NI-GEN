use std::collections::{HashMap, HashSet};

use crate::{
    analysis::cubicsolver::CubicSolver,
    inst::{Reg, RegReg, RegRegImm},
    ir::{BasicBlock, Function, InstUUID, Instruction, Register},
};

/// Position of the alloca
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum Cell {
    Alloc(InstUUID),
    Volatile,
}

#[derive(Clone, PartialEq, Eq, Hash)]
enum Place {
    Register(Register),
    Memory(Register),
}

impl Place {
    fn is_register(&self) -> bool {
        if let Place::Register(_) = self {
            true
        } else {
            false
        }
    }

    fn get_reg(&self) -> Register {
        match self {
            &Place::Register(reg) => reg,
            &Place::Memory(reg) => reg,
        }
    }
}

pub struct AndersenAnalysis<'a> {
    function: &'a Function,
    memory: Option<Vec<Register>>,
}

impl<'a> AndersenAnalysis<'a> {
    pub fn new(function: &'a Function) -> Self {
        Self {
            function,
            memory: None,
        }
    }

    pub fn analyze(&mut self) -> HashMap<Register, HashSet<Cell>> {
        let mut solver: CubicSolver<Cell, Place> = CubicSolver::new();

        for bb in self.function.blocks.iter() {
            self.analyze_bb(bb, &mut solver);
        }

        solver
            .solution()
            .into_iter()
            .filter(|(place, _)| place.is_register())
            .map(|(place, cell)| (place.get_reg(), cell))
            .collect()
    }

    fn analyze_bb(&mut self, bb: &BasicBlock, solver: &mut CubicSolver<Cell, Place>) {
        for inst in bb.iter() {
            self.analyze_inst(inst, solver);
        }
    }

    fn analyze_inst(&mut self, inst: &Instruction, solver: &mut CubicSolver<Cell, Place>) {
        match inst.data {
            crate::inst::InstructionType::Arg(_) => {
                solver.includes(Cell::Volatile, Place::Register(inst.id))
            }
            crate::inst::InstructionType::Alloca(_) => {
                solver.includes(Cell::Alloc(inst.id), Place::Register(inst.id))
            }
            crate::inst::InstructionType::Mov(Reg(reg)) => {
                solver.add_edge(Place::Register(reg), Place::Register(inst.id))
            }
            crate::inst::InstructionType::Gep(_, RegRegImm(start, _, _)) => {
                solver.add_edge(Place::Register(start), Place::Register(inst.id))
            }
            // anything that could be in the value it the
            // address of the [reg] could be also in the inst.id
            crate::inst::InstructionType::Ld(Reg(addr)) => {
                // every thing that could be on the address [addr] could be in the register
                solver.add_edge(Place::Memory(addr), Place::Register(inst.id));

                for mem in self.get_memory() {
                    solver.includes_implies(
                        Cell::Alloc(mem),
                        Place::Memory(addr),
                        Place::Memory(mem),
                        Place::Register(inst.id),
                    );
                }
            }
            // anything that could be in the register reg
            // could be in the in the memory on the address [addr]
            crate::inst::InstructionType::St(RegReg(addr, reg)) => {
                solver.add_edge(Place::Register(reg), Place::Memory(addr));
                for mem in self.get_memory() {
                    solver.includes_implies(
                        Cell::Alloc(mem),
                        Place::Memory(addr),
                        Place::Register(reg),
                        Place::Memory(mem),
                    );
                }
            }
            // I am still not using this so fuck it
            crate::inst::InstructionType::Allocg(_) => todo!(),
            _ => (),
        }
    }

    fn get_memory(&mut self) -> Vec<Register> {
        if let Some(memory) = &self.memory {
            memory.clone()
        } else {
            let memory: HashSet<Register> = self
                .function
                .blocks
                .iter()
                .map(|bb| {
                    bb.iter().filter(|inst| match inst.data {
                        crate::inst::InstructionType::St(_) => true,
                        crate::inst::InstructionType::Ld(_) => true,
                        _ => false,
                    })
                })
                .flatten()
                .map(|inst| {
                    match inst.data {
                        crate::inst::InstructionType::St(RegReg(addr, _)) => addr,
                        crate::inst::InstructionType::Ld(Reg(addr)) => addr,
                        _ => unreachable!()
                    }
                })
                .collect();
            self.memory = Some(memory.into_iter().collect());
            self.get_memory()
        }
    }
}
