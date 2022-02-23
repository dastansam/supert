use std::result;

/// Data type that represents a Bytecode interpreter.
/// 
/// A program is a sequence of instructions. Interpreter is stack based, rather than register based.
pub struct SupertVM {
    pub instructions: Vec<Instruction>,
    pub stack: Vec<i32>,
}

/// Types of instructions that can be performed on the stack.
pub enum Instruction {
    LoadVal(i32),
    WriteVar(String),
    ReadVar(String),
    LoopStart,
    LoopEnd,
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Finish,
}

