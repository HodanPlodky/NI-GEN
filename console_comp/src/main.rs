use std::{
    collections::{HashMap, HashSet},
    env, fs,
};

use backend::{asm_compile, emit::emit_assembly};
use frontend::parse;
use middleend::{
    analysis::{
        analysis::analyze_program,
        anderson::AndersenAnalysis,
        const_mem::{ConstantMemoryAnalysis, MemoryPlace},
        lattice::FlatElem,
        live::LiveRegisterAnalysis,
        possible_mem::PossibleMemAnalysis,
    },
    ir::Register,
    ir_compile::ir_compile,
    ir_interpret::run,
};

fn printlive(result: HashMap<String, Vec<Vec<HashSet<Register>>>>) {
    for (name, data) in result.iter() {
        println!("function {} {{", name);
        for bb_index in 0..data.len() {
            println!("BB{}:", bb_index);
            for inst_state in data[bb_index].iter() {
                println!("\t{:?}", inst_state);
            }
        }
        println!("}}\n");
    }
}

fn printconst(
    result: HashMap<String, Vec<Vec<HashMap<MemoryPlace, FlatElem<(bool, usize, usize)>>>>>,
) {
    for (name, data) in result.iter() {
        println!("function {} {{", name);
        for bb_index in 0..data.len() {
            println!("BB{}:", bb_index);
            for inst_state in data[bb_index].iter() {
                println!("\t{:?}", inst_state);
            }
        }
        println!("}}\n");
    }
}

fn printposs(result: HashMap<String, Vec<Vec<HashMap<MemoryPlace, HashSet<Register>>>>>) {
    for (name, data) in result.iter() {
        println!("function {} {{", name);
        for bb_index in 0..data.len() {
            println!("BB{}:", bb_index);
            for inst_state in data[bb_index].iter() {
                println!(
                    "\t{:?}",
                    inst_state
                        .into_iter()
                        .map(|(MemoryPlace(m), s)| (m.clone(), s.clone()))
                        .collect::<Vec<(Register, HashSet<Register>)>>()
                );
            }
        }
        println!("}}\n");
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Wrong number of args");
        return;
    }
    let path: String = args[2].clone();
    let content: String = fs::read_to_string(path.clone()).unwrap();

    let prog = parse(content, path).unwrap();

    if args[1] == "--parse" {
        println!("{:?}", prog);
        return;
    }

    let ir_prog = ir_compile(prog).unwrap();
    if args[1] == "--ir" {
        println!("{}", ir_prog);
        let res = run(ir_prog).unwrap();
        println!("{}", res);
    } else if args[1] == "--asm" {
        let asm_prog = asm_compile(ir_prog);
        let asm_text = emit_assembly(asm_prog);
        println!("{}", asm_text);
    } else if args[1] == "--live" {
        println!("{}", ir_prog);
        let live = LiveRegisterAnalysis::new(&ir_prog.funcs.get(&"main".to_string()).unwrap());
        let result = analyze_program(&ir_prog, live);
        printlive(result);
    } else if args[1] == "--point" {
        println!("{}", ir_prog);
        for (name, f) in ir_prog.funcs.iter() {
            let mut andersen = AndersenAnalysis::new(f);
            let result = andersen.analyze();

            println!("{name}:");
            for (reg, cell) in &result {
                println!("\t{:?} -> {:?}", reg, cell);
            }
        }
    } else if args[1] == "--const" {
        println!("{}", ir_prog);
        let const_analysis =
            ConstantMemoryAnalysis::new(&ir_prog.funcs.get(&"main".to_string()).unwrap());
        let result = analyze_program(&ir_prog, const_analysis);
        printconst(result);
    } else if args[1] == "--pred" {
        println!("{}", ir_prog);
        for (name, func) in ir_prog.funcs.iter() {
            println!("function {} {{", name);
            for bb in func.blocks.iter() {
                println!("\t{:?}", bb.pred());
            }
            println!("}}");
        }
    } else if args[1] == "--poss" {
        println!("{}", ir_prog);
        let poss_analysis =
            PossibleMemAnalysis::new(&ir_prog.funcs.get(&"main".to_string()).unwrap());
        let result = analyze_program(&ir_prog, poss_analysis);
        printposs(result);
    }
}
