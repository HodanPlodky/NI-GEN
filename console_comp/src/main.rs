use std::{env, fs};

use backend::{asm_compile, emit::emit_assembly};
use frontend::parse;
use middleend::{ir_compile::ir_compile, ir_interpret::run};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Wrong number of args");
        return;
    }
    let path: String = args[1].clone();
    let content: String = fs::read_to_string(path.clone()).unwrap();

    let prog = parse(content, path).unwrap();
    let ir_prog = ir_compile(prog).unwrap();
    //println!("{}", ir_prog);
    //let res = run(ir_prog.).unwrap();
    //println!("{}", res);
    
    let asm_prog = asm_compile(ir_prog);
    let asm_text = emit_assembly(asm_prog);
    println!("{}", asm_text);
}
