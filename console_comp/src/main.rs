use middleend::{ir::{IrBuilder, FunctionBuilder, I}, inst::{Terminator, RegType, ImmI, Reg, ImmC}, ir_interpret::run};

fn main() {
    let mut builder = IrBuilder::default();
    builder.add(I::Ret(Terminator), RegType::Void);
    let mut fn_b = FunctionBuilder::new(0, RegType::Void);
    let reg_n = fn_b.add(I::Ldi(ImmI(5)), RegType::Int);
    let reg_c = fn_b.add(I::Ldc(ImmC('\n')), RegType::Int);
    fn_b.add(I::Print(Reg(reg_n)), RegType::Void);
    fn_b.add(I::Print(Reg(reg_c)), RegType::Void);
    fn_b.add(I::Ret(Terminator), RegType::Void);
    builder.add_fn("main", fn_b.create()).unwrap();
        
    run(builder.create()).unwrap();
    println!("Hello, world!");
}
