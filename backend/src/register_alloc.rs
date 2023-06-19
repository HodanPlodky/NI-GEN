#[derive(Clone, Copy)]
pub enum ValueCell {
    Register(usize),
    StackOffset(i64),
}

pub trait RegAllocator {
    fn get_location(&self, reg: middleend::inst::Register) -> ValueCell;
    fn get_used(&self) -> &Vec<usize>;
}

/*
 * First gets the space and has it to the end of the 
 * usage of this allocator
 */
pub struct NaiveAllocator;

/*
 * First gets the space but it only has
 * is for a duration of the lifetime of the ir register
 */
pub struct LinearAllocator;

/*
 * This one is a big boy
 */
pub struct ColoringAllocator;
