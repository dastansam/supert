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

/// Program result.
pub enum ProgramResult {
    Ok,
    CompileError,
    RuntimeError,
}

impl SupertVM {
    pub fn new() -> SupertVM {
        SupertVM {
            instructions: Vec::new(),
            stack: Vec::new(),
        }
    }

    pub(crate) fn interpret(mut self) -> ProgramResult {
        loop {
            match self.instructions.pop() {
                Some(Instruction::LoadVal(val)) => {
                    self.stack.push(val);
                },
                Some(Instruction::WriteVar(var)) => {
                    let val = self.stack.pop().unwrap();
                    println!("{} = {}", var, val);
                },
                Some(Instruction::ReadVar(var)) => {
                    let mut input = String::new();
                    std::io::stdin().read_line(&mut input).unwrap();
                    let val = input.trim().parse::<i32>().unwrap();
                    self.stack.push(val);
                },
                Some(Instruction::LoopStart) => {
                    let val = self.stack.pop().unwrap();
                    if val == 0 {
                        self.instructions.pop();
                    }
                },
                Some(Instruction::LoopEnd) => {
                    let val = self.stack.pop().unwrap();
                    self.instructions.push(Instruction::LoopStart);
                    self.instructions.push(Instruction::LoadVal(val));
                },
                Some(Instruction::Add) => {
                    let val2 = self.stack.pop().unwrap();
                    let val1 = self.stack.pop().unwrap();
                    self.stack.push(val1 + val2);
                },
                Some(Instruction::Sub) => {
                    let val2 = self.stack.pop().unwrap();
                    let val1 = self.stack.pop().unwrap();
                    self.stack.push(val1 - val2);
                },
                Some(Instruction::Mul) => {
                    let val2 = self.stack.pop().unwrap();
                    let val1 = self.stack.pop().unwrap();
                    self.stack.push(val1 * val2);
                },
                Some(Instruction::Div) => {
                    let val2 = self.stack.pop().unwrap();
                    let val1 = self.stack.pop().unwrap();
                    self.stack.push(val1 / val2);
                },
                Some(Instruction::Mod) => {
                    let val2 = self.stack.pop().unwrap();
                    let val1 = self.stack.pop().unwrap();
                    self.stack.push(val1 % val2);
                },
                Some(Instruction::Finish) => {
                    return ProgramResult::Ok;
                },
                None => {
                    return ProgramResult::RuntimeError;
                },
                _ => {
                    return ProgramResult::CompileError;
                },
            }
        }
    }
}
