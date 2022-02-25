/// - No, just interpreter.
/// - If you can manage functions and inputs, yes.
/// - Flat is as a single enum without nested enums, keep it simple.

use std::{collections::HashMap};

/// Maximum stack size: 2^16 - 1
const MAX_STACK_SIZE: usize = 65535;

/// Data type that represents a Bytecode interpreter.
/// 
/// A program is a sequence of instructions. Interpreter is stack based, rather than register based.
#[derive(Clone)]
pub struct Bytecode {
    // List of instructions
    pub instructions: Vec<Instruction>,
    // Program stack
    pub stack: Vec<i64>,
    /// Mapping for local variables
    pub variables: HashMap<String, i64>,
    /// Current instruction pointer, points to the next instruction to be executed
    pub ip: usize,
    /// Current stack pointer, always points to the top of the stack
    pub sp: usize,
}

/// Types of instructions that can be performed on the stack.
#[derive(Clone)]
pub enum Instruction {
    /// Push a value onto the stack
    LoadVal(i64),
    /// Write value at top of stack to a variable
    WriteVar(String),
    /// Read value from a variable
    ReadVar(String),
    /// Add top two values on stack
    Add,
    /// Subtract top two values on stack
    Sub,
    /// Multiply top two values on stack
    Mul,
    /// Divide top two values on stack
    Div,
    /// Modulo top two values on stack
    Mod,
    /// Jump to a specific instruction
    Jump(u8),
    /// Jump to a specific instruction if top of stack is zero
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

impl Bytecode {
    pub fn new(instructions: Vec<Instruction>) -> Bytecode {
        Bytecode {
            instructions,
            stack: Vec::new(),
            variables: HashMap::new(),
            ip: 0,
            sp: 0,
        }
    }

    /// Push a value onto the stack
    fn push_val(mut self, val: i64) -> Result<(), VMError> {
        if self.sp < MAX_STACK_SIZE {
            self.stack.push(val);
            self.sp += 1;
            Ok(())
        } else {
            Err(VMError::StackOverflow)
        }
    }

    /// Pop a value from the stack
    fn pop_val(mut self) -> Result<i64, VMError> {
        if self.sp > 0 {
            self.sp -= 1;
            Ok(self.stack.pop().unwrap())
        } else {
            Err(VMError::StackUnderflow)
        }
    }

    /// Interprets the program.
    /// 
    /// Runs insructions one by one.
    pub fn interpret(mut self) -> Result<i64, VMError> {
        loop {
            let instruction_res = match self.instructions.clone().get(self.ip) {
                Some(instruction) => {
                    match instruction {
                        Instruction::LoadVal(val) => {
                            self.push_val(*val)?;
                            None
                        },
                        Instruction::WriteVar(var) => {
                            match self.pop_val() {
                                Ok(val) => {
                                    self.variables.insert(var.to_string(), val);
                                    None
                                },
                                _ => Some(VMError::StackUnderflow),
                            }
                        },
                        Instruction::ReadVar(var) => {
                            match self.variables.get(&var.to_string()) {
                                Some(val) => {
                                    self.push_val(*val)?;
                                    None
                                },
                                _ => Some(VMError::StackUnderflow),
                            }
                        },
                        Instruction::Jump(offset) => {
                            // let val = self.stack.pop().unwrap();
                            self.ip += *offset as usize;
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
                    }
                },
                None => break,
            };

            // If instruction fails to execute, return error.
            if let Some(err) = instruction_res {
                return Err(err);
            }
        }

        match self.pop_val() {
            Ok(result) => Ok(result),
            Err(e) => Err(e),
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

        // var a = 3 + 4 * 5 - 3;
        // var b = a / 2;
        // b % 3 => 1
        let mut vm_1 = SupertVM::new(Default::default());
        vm_1.instructions.push(Instruction::LoadVal(3));
        vm_1.instructions.push(Instruction::LoadVal(4));
        vm_1.instructions.push(Instruction::LoadVal(5));
        vm_1.instructions.push(Instruction::Mul);
        vm_1.instructions.push(Instruction::Add);
        vm_1.instructions.push(Instruction::LoadVal(3));
        vm_1.instructions.push(Instruction::Sub);
        vm_1.instructions.push(Instruction::WriteVar("a".to_string()));
        vm_1.instructions.push(Instruction::LoadVal(2));
        vm_1.instructions.push(Instruction::ReadVar("a".to_string()));
        vm_1.instructions.push(Instruction::Div);
        vm_1.instructions.push(Instruction::WriteVar("b".to_string()));
        vm_1.instructions.push(Instruction::ReadVar("b".to_string()));
        vm_1.instructions.push(Instruction::LoadVal(3));
        vm_1.instructions.push(Instruction::Mod);
        vm_1.instructions.push(Instruction::Finish);
        assert_eq!(vm_1.interpret().unwrap_or(0), 1);
        
    }
}