
use crate::method::{ Instruction };

/// Maximum stack size: 2^16 - 1
const MAX_STACK_SIZE: usize = 65535;

/// Data type that represents a Bytecode interpreter.
/// 
/// A program is a sequence of instructions. Interpreter is stack based, rather than register based.
#[derive(Clone)]
pub struct Bytecode {
    /// Instructions bytecode
    pub instructions: Vec<u8>,
    /// Program stack
    pub stack: Vec<i64>,
    /// Method name mapped to start instruction pointer
    pub method_labes: HashMap<String, usize>,
    /// Mapping for local variables
    pub variables: HashMap<String, i64>,
    /// Current instruction pointer, points to the next instruction to be executed
    pub ip: usize
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

use std::{collections::HashMap};

/// Macro for executing native operations
/// +, -, *, /, %, ==, !=, >, <, >=, <=
macro_rules! execute_native {
    ($supert_vm:expr, $opcode:tt) => {{
        match $supert_vm.stack.pop() {
            Some(a) => {
                match $supert_vm.stack.pop() {
                    Some(b) => {
                        $supert_vm.stack.push((b $opcode a) as i64);
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
    pub fn new(instructions: Vec<u8>) -> Bytecode {
        Bytecode {
            instructions,
            method_labes: HashMap::new(),
            stack: Vec::new(),
            variables: HashMap::new(),
            ip: 0,
        }
    }

    /// Get next instruction from the program
    fn next_instruction(&mut self) -> Option<Instruction> {
        if self.ip >= self.instructions.len() {
            return None;
        }
        let instruction = self.instructions[self.ip];
        println!("Current ip: {} Instruction: {:?}", self.ip, instruction);
        self.ip += 1;
        Some(Instruction::from(instruction))
    }

    /// Push a value onto the stack
    fn push_val(&mut self, val: i64) -> Result<(), VMError> {
        dbg!("Pushing value onto stack");
        if self.stack.len() < MAX_STACK_SIZE {
            self.stack.push(val);
            dbg!("Current stack: {}", self.stack.clone());
            Ok(())
        } else {
            Err(VMError::StackOverflow)
        }
    }

    /// Pop a value from the stack
    fn pop_val(&mut self) -> Result<i64, VMError> {
        println!("Stack {:?}", self.stack.clone());
        if self.stack.len() > 0 {
            Ok(self.stack.pop().unwrap())
        } else {
            Err(VMError::StackUnderflow)
        }
    }

    /// Read next string from the program
    /// Variable names are strictly 4 character long
    fn read_string(&mut self) -> Result<String, VMError> {
        let string = self.instructions[self.ip..self.ip + 4]
            .iter()
            .map(|&byte| byte as char)
            .collect::<String>();
        self.ip += 4;
        Ok(string)
    }

    /// Read next byte from the program
    fn read_byte(&mut self) -> Result<u8, VMError> {
        let byte = self.instructions[self.ip];
        self.ip += 1;
        Ok(byte)
    }

    /// Read next short integer from the program
    fn read_short(&mut self) -> Result<i16, VMError> {
        match self.instructions[self.ip..self.ip+2].try_into() {
            Ok(val) => {
                self.ip += 2;
                Ok(i16::from_le_bytes(val))
            },
            Err(_) => Err(VMError::UnknownInstruction),
        }
    }

    /// Read next short integer from the program
    fn read_i32(&mut self) -> Result<i32, VMError> {
        match self.instructions[self.ip..self.ip+4].try_into() {
            Ok(val) => {
                self.ip += 4;
                Ok(i32::from_le_bytes(val))
            },
            Err(_) => Err(VMError::UnknownInstruction),
        }
    }

    /// Read next long integer from the program
    fn read_long(&mut self) -> Result<i64, VMError> {
        match self.instructions[self.ip..self.ip + 8].try_into() {
            Ok(val) => {
                println!("Reading long {:?}", val);
                self.ip += 8;
                Ok(i64::from_le_bytes(val))
            },
            Err(_) => Err(VMError::StackOverflow),
        }
    }

    /// Interprets the program.
    /// 
    /// Runs insructions one by one.
    pub fn interpret(&mut self) -> Result<i64, VMError> {
        println!("Instructions: {:?}", self.instructions.clone());
        loop {
            let current_instruction = self.next_instruction();
            let instruction_res = match current_instruction {
                Some(instruction) => {
                    match instruction {
                        Instruction::LoadVal => {
                            let val = self.read_long()?;
                            println!("LoadVal: {}", val);
                            self.push_val(val)?;
                            None
                        },
                        Instruction::WriteVar => {
                            let var_name = self.read_string()?;
                            println!("Varname {}", var_name);
                            let val = self.pop_val()?;
                            println!("Val {}", val);
                            self.variables.insert(var_name, val);
                            None
                        },
                        Instruction::ReadVar => {
                            let var_name = self.read_string()?;
                            match self.variables.get(&var_name) {
                                Some(val) => {
                                    println!("Pushing var {}", val);
                                    self.push_val(*val)?;
                                    None
                                },
                                _ => Some(VMError::StackUnderflow),
                            }
                        },
                        Instruction::FuncCall => {
                            unimplemented!()
                        },
                        Instruction::Jump => {
                            let offset = self.read_byte()? as usize;
                            self.ip += offset;
                            None
                        },
                        Instruction::JumpBack => {
                            let offset = self.read_byte()? as usize;
                            println!("JumpBack {}", offset);
                            self.ip -= offset;
                            None
                        },
                        Instruction::JumpIfFalse => {
                            let offset = self.read_byte()? as usize;
                            let val = self.pop_val()?;
                            println!("JumpIfFalse: {}", val);
                            if val == 0 {
                                self.ip += offset;
                            }
                            None
                        },
                        Instruction::JumpIfTrue => {
                            let offset = self.read_byte()? as usize;
                            let val = self.pop_val()?;
                            if val != 0 {
                                self.ip += offset;
                            }
                            None
                        },
                        Instruction::Add => execute_native!(self, +),
                        Instruction::Sub => execute_native!(self, -),
                        Instruction::Mul => execute_native!(self, *),
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
                        Instruction::Mod => execute_native!(self, %),
                        Instruction::Eq => execute_native!(self, ==),
                        Instruction::NotEq => execute_native!(self, !=),
                        Instruction::Lt => execute_native!(self, <),
                        Instruction::Gt => execute_native!(self, >),
                        Instruction::Lte => execute_native!(self, <=),
                        Instruction::Gte => execute_native!(self, >=),
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
    fn test_arithmetic() {
        // Arithmetic
        // let x = 1
        // let y = 2
        // return (x + 1) * y
        let mut vm = Bytecode::new(vec![
            Instruction::LoadVal.into(), 0x01, 0, 0, 0, 0, 0, 0, 0,
            Instruction::WriteVar.into(), 0x78, 0x00, 0x00, 0x00,
            Instruction::LoadVal.into(), 0x02, 0, 0, 0, 0, 0, 0, 0,
            Instruction::WriteVar.into(), 0x79, 0x00, 0x00, 0x00,
            Instruction::ReadVar.into(), 0x78, 0x00, 0x00, 0x00,
            Instruction::LoadVal.into(), 0x01, 0, 0, 0, 0, 0, 0, 0,
            Instruction::Add.into(),
            Instruction::ReadVar.into(), 0x79, 0x00, 0x00, 0x00,
            Instruction::Mul.into(),
            Instruction::Finish.into(),
        ]);

        assert_eq!(vm.interpret().unwrap(), 4);
            
        // Arithmetic case 2
        // let x = 5
        // let y = 8
        // let z = x * y
        // z / 2
        let mut vm_1 = Bytecode::new(vec![
            Instruction::LoadVal.into(), 0x05, 0, 0, 0, 0, 0, 0, 0,
            Instruction::WriteVar.into(), 0x78, 0x00, 0x00, 0x00,
            Instruction::LoadVal.into(), 0x08, 0, 0, 0, 0, 0, 0, 0,
            Instruction::WriteVar.into(), 0x79, 0x00, 0x00, 0x00,
            Instruction::ReadVar.into(), 0x78, 0x00, 0x00, 0x00,
            Instruction::ReadVar.into(), 0x79, 0x00, 0x00, 0x00,
            Instruction::Mul.into(),
            Instruction::WriteVar.into(), 0x7A, 0x00, 0x00, 0x00,
            Instruction::LoadVal.into(), 0x02, 0, 0, 0, 0, 0, 0, 0,
            Instruction::ReadVar.into(), 0x7A, 0x00, 0x00, 0x00,
            Instruction::Div.into(),
            Instruction::Finish.into(),
        ]);

        assert_eq!(vm_1.interpret().unwrap(), 20);
    }

    #[test]
    fn test_zero_division() {
        let mut vm = Bytecode::new(vec![
            Instruction::LoadVal.into(), 0x00, 0, 0, 0, 0, 0, 0, 0,
            Instruction::LoadVal.into(), 0x01, 0, 0, 0, 0, 0, 0, 0,
            Instruction::Div.into(),
            Instruction::Finish.into(),
        ]);

        assert_eq!(vm.interpret().unwrap_err(), VMError::DivisionByZero);
    }

    #[test]
    fn test_loop() {
        // Pseudocode is this:
        //
        // let test = 1 + 5
        // while test < 10:
        //    test += 1
        // i
        let mut vm = Bytecode::new(vec![
            Instruction::LoadVal.into(), 0x01, 0, 0, 0, 0, 0, 0, 0, // 1
            Instruction::LoadVal.into(), 0x05, 0, 0, 0, 0, 0, 0, 0, // 5
            Instruction::Add.into(), // 6
            Instruction::WriteVar.into(), 74, 65, 73, 74, // "test"
            Instruction::ReadVar.into(), 74, 65, 73, 74, // "test"
            Instruction::LoadVal.into(), 0x0A, 0, 0, 0, 0, 0, 0, 0, // 10
            Instruction::Lt.into(), // test < 10
            Instruction::JumpIfFalse.into(), 0x16, // Skip to instruction after `JumpBack` if false
            Instruction::ReadVar.into(), 74, 65, 73, 74, // "test"
            Instruction::LoadVal.into(), 0x01, 0, 0, 0, 0, 0, 0, 0, // 1
            Instruction::Add.into(), // test += 1
            Instruction::WriteVar.into(), 74, 65, 73, 74, // "test"
            Instruction::JumpBack.into(), 0x27, // Jump back to loop condition
            Instruction::ReadVar.into(), 74, 65, 73, 74, // "test"
            Instruction::Finish.into(), 
        ]);

        assert_eq!(vm.interpret().unwrap(), 10);
    }

    #[test]
    fn test_for_loop() {
        // Sum of all squares of numbers from 1 to 10
        // let test = 0
        // for temp in 1..11:
        //    test += temp * temp
        // test => 385
        let mut vm = Bytecode::new(vec![
            Instruction::LoadVal.into(), 0, 0, 0, 0, 0, 0, 0, 0,
            Instruction::WriteVar.into(), 0x74, 0x65, 0x73, 0x74, // "test"
            Instruction::LoadVal.into(), 0x01, 0, 0, 0, 0, 0, 0, 0,
            Instruction::WriteVar.into(), 0x74, 0x65, 0x6d, 0x70, // "temp"
            Instruction::LoadVal.into(), 0x0A, 0, 0, 0, 0, 0, 0, 0, // 10
            Instruction::ReadVar.into(), 0x74, 0x65, 0x6d, 0x70, // "temp"
            Instruction::Gte.into(), // temp <= 10
            Instruction::JumpIfFalse.into(), 44, // Skip 44 instructions forward (to after JUMP_BACK)
            Instruction::ReadVar.into(), 0x74, 0x65, 0x6d, 0x70,
            Instruction::ReadVar.into(), 0x74, 0x65, 0x6d, 0x70,
            Instruction::Mul.into(),
            Instruction::ReadVar.into(), 0x74, 0x65, 0x73, 0x74,
            Instruction::Add.into(),
            Instruction::WriteVar.into(), 0x74, 0x65, 0x73, 0x74,
            Instruction::ReadVar.into(), 0x74, 0x65, 0x6d, 0x70, // "temp"
            Instruction::LoadVal.into(), 0x01, 0, 0, 0, 0, 0, 0, 0, // 1
            Instruction::Add.into(), // temp++
            Instruction::WriteVar.into(), 0x74, 0x65, 0x6d, 0x70, // "temp"
            Instruction::JumpBack.into(), 61, // Jump back to the start of the loop (condition)
            Instruction::ReadVar.into(), 0x74, 0x65, 0x73, 0x74, 
            Instruction::Finish.into(),
        ]);

        assert_eq!(vm.interpret().unwrap(), 385);
    }
}
