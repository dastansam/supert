use std::{collections::HashMap};

/// Data type that represents a Bytecode interpreter.
/// 
/// A program is a sequence of instructions. Interpreter is stack based, rather than register based.
pub struct SupertVM {
    // List of instructions
    pub instructions: Vec<Instruction>,
    // Program stack
    pub stack: Vec<i64>,
    /// Mapping for local variables
    pub memory: HashMap<String, i64>,
}

/// Types of instructions that can be performed on the stack.
pub enum Instruction {
    LoadVal(i64),
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


/// VM error type
#[derive(Debug, PartialEq)]
pub enum VMError {
    DivisionByZero,
    StackOverflow,
    StackUnderflow,
    UnknownInstruction,
}

/// Program result.
pub enum ProgramResult {
    Ok,
    CompileError,
    RuntimeError,
    ProgramError(VMError),
}

macro_rules! execute_opcode {
    ($supert_vm:expr, $opcode:tt) => {{
        match $supert_vm.stack.pop() {
            Some(a) => {
                match $supert_vm.stack.pop() {
                    Some(b) => {
                        $supert_vm.stack.push(b $opcode a);
                        None
                    },
                    _ => Some(VMError::StackUnderflow),
                }
            },
            _ => Some(VMError::StackUnderflow),
        }
    }}
}

impl SupertVM {
    pub fn new(instructions: Vec<Instruction>) -> SupertVM {
        SupertVM {
            instructions,
            stack: Vec::new(),
            memory: HashMap::new(),
        }
    }

    /// Interprets the program.
    /// 
    /// Runs insructions one by one.
    pub fn interpret(mut self) -> Result<i64, VMError> {
        for instruction in self.instructions {
            let instruction_res = match instruction {
                Instruction::LoadVal(val) => {
                    self.stack.push(val);
                    None
                },
                Instruction::WriteVar(var) => {
                    match self.stack.pop() {
                        Some(val) => {
                            self.memory.insert(var, val);
                            None
                        },
                        _ => Some(VMError::StackUnderflow),
                    }
                },
                Instruction::ReadVar(var) => {
                    match self.memory.get(&var) {
                        Some(val) => {
                            self.stack.push(*val);
                            None
                        },
                        _ => Some(VMError::StackUnderflow),
                    }
                },
                Instruction::LoopStart => {
                    // let val = self.stack.pop().unwrap();
                    None
                },
                Instruction::LoopEnd => {
                    // let val = self.stack.pop().unwrap();
                    None
                },
                Instruction::Add => execute_opcode!(self, +),
                Instruction::Sub => execute_opcode!(self, -),
                Instruction::Mul => execute_opcode!(self, *),
                Instruction::Div => {
                    if let (Some(a), Some(b)) = (self.stack.pop(), self.stack.pop()) {
                        if b == 0 {
                            Some(VMError::DivisionByZero)
                        } else {
                            self.stack.push(a / b);
                            None
                        }
                    } else {
                        Some(VMError::StackUnderflow)
                    }
                },
                Instruction::Mod => execute_opcode!(self, %),
                Instruction::Finish => break,
            };

            // If instruction fails to execute, return error.
            if let Some(err) = instruction_res {
                return Err(err);
            }
        }

        match self.stack.pop() {
            Some(result) => Ok(result),
            None => Err(VMError::StackUnderflow),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_add() {
        let vm = SupertVM::new(vec![
            Instruction::LoadVal(5),
            Instruction::LoadVal(6),
            Instruction::Add,
        ]);
        assert_eq!(vm.interpret().unwrap_or(0), 11);
    }

    #[test]
    fn test_var_add() {
        let vm = SupertVM::new(vec![
            Instruction::LoadVal(5),
            Instruction::WriteVar("a".to_string()),
            Instruction::LoadVal(6),
            Instruction::WriteVar("b".to_string()),
            Instruction::ReadVar("a".to_string()),
            Instruction::ReadVar("b".to_string()),
            Instruction::Add,
        ]);
        assert_eq!(vm.interpret().unwrap_or(0), 11);
    }

    #[test]
    fn test_simple_sub() {
        let vm = SupertVM::new(vec![
            Instruction::LoadVal(5),
            Instruction::LoadVal(6),
            Instruction::Sub,
        ]);
        assert_eq!(vm.interpret().unwrap_or(0), -1);
    }

    #[test]
    fn test_simple_mul() {
        let vm = SupertVM::new(vec![
            Instruction::LoadVal(5),
            Instruction::LoadVal(6),
            Instruction::Mul,
        ]);
        assert_eq!(vm.interpret().unwrap_or(0), 30);
    }

    #[test]
    fn test_simple_div() {
        let vm = SupertVM::new(vec![
            Instruction::LoadVal(6),
            Instruction::LoadVal(15),
            Instruction::Div,
        ]);
        assert_eq!(vm.interpret().unwrap_or(0), 2);

        // Test division by zero
        let vm = SupertVM::new(vec![
            Instruction::LoadVal(0),
            Instruction::LoadVal(5),
            Instruction::Div,
        ]);
        assert_eq!(vm.interpret().err(), Some(VMError::DivisionByZero));
    }

    #[test]
    fn test_arithmetic_expression() {
        // 1 + 3 * 4 - 5
        let mut vm = SupertVM::new(Default::default());
        vm.instructions.push(Instruction::LoadVal(1));
        vm.instructions.push(Instruction::LoadVal(3));
        vm.instructions.push(Instruction::LoadVal(4));
        vm.instructions.push(Instruction::Mul);
        vm.instructions.push(Instruction::Add);
        vm.instructions.push(Instruction::LoadVal(5));
        vm.instructions.push(Instruction::Sub);
        vm.instructions.push(Instruction::Finish);

        let result = vm.interpret().unwrap_or(0);
        assert_eq!(result, 8);
    }
}