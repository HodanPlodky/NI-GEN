use std::collections::{HashMap, HashSet};

use crate::{
    analysis::cubicsolver::CubicSolver,
    ir::{BasicBlock, Function, Instruction, Register}, inst::Reg,
};

/// Position of the alloca
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Cell(usize, usize);

pub struct AndersenAnalysis<'a> {
    function: &'a Function,
}

impl<'a> AndersenAnalysis<'a> {
    pub fn new(function: &'a Function) -> Self {
        Self { function }
    }

    pub fn analyze(&self) -> HashMap<Register, HashSet<Cell>> {
        let mut solver: CubicSolver<Cell, Register> = CubicSolver::new();

        for bb in self.function.blocks.iter() {
            self.analyze_bb(bb, &mut solver);
        }

        solver.solution()
    }

    fn analyze_bb(&self, bb: &BasicBlock, solver: &mut CubicSolver<Cell, Register>) {
        for inst in bb.iter() {
            self.analyze_inst(inst, solver);
        }
    }

    fn analyze_inst(&self, inst: &Instruction, solver: &mut CubicSolver<Cell, Register>) {
        match inst.data {
            crate::inst::InstructionType::Alloca(_) => {
                solver.includes(Cell(inst.id.1, inst.id.2), inst.id)
            }
            crate::inst::InstructionType::Mov(Reg(reg)) => solver.add_edge(reg, inst.id),
            crate::inst::InstructionType::Ld(_) => todo!(),
            crate::inst::InstructionType::St(_) => todo!(),
            crate::inst::InstructionType::Allocg(_) => todo!(),
            _ => ()
        }
    }
}
