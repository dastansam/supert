
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;
use std::{collections::HashMap};

use crate::error::VMError;
use crate::stack::StackValue;
use crate::instruction::{ Instruction };

/// Maximum stack size: 2^16 - 1
const MAX_STACK_SIZE: usize = 65535;

/// Data type that represents a Bytecode interpreter.
/// 
/// A program is a sequence of instructions. Interpreter is stack based, rather than register based.
#[derive(Debug)]
pub struct Bytecode {
    /// Instructions bytecode
    pub instructions: Vec<Instruction>,
    /// Program stack
    pub stack: Vec<StackValue>,
    /// Method name mapped to start instruction pointer
    pub method_labels: HashMap<String, usize>,
    /// Mapping for local variables
    pub variables: HashMap<String, i64>,
    /// Current instruction pointer, points to the next instruction to be executed
    pub ip: usize
}

/// Macro for executing native operations
/// +, -, *, /, %, ==, !=, >, <, >=, <=
macro_rules! execute_native {
    ($supert_vm:expr, $opcode:tt) => {{
        match $supert_vm.pop_val() {
            Ok(a) => {
                match $supert_vm.pop_val() {
                    Ok(b) => {
                        $supert_vm.stack.push(StackValue::Int((b $opcode a) as i64));
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
            method_labels: HashMap::new(),
            stack: Vec::new(),
            variables: HashMap::new(),
            ip: 0,
        }
    }

    /// Get next instruction from the program
    fn next_instruction(mut self) -> Option<Instruction> {
        if self.ip >= self.instructions.len() {
            return None;
        }
        let instruction = &self.instructions[self.ip];
        println!("Current ip: {} Instruction: {:?}", self.ip, instruction);
        self.ip += 1;
        Some(*instruction)
    }

    /// Push a value onto the stack
    fn push_val(mut self, val: StackValue) -> Result<(), VMError> {
        if self.stack.len() < MAX_STACK_SIZE {
            self.stack.push(val);
            Ok(())
        } else {
            Err(VMError::StackOverflow)
        }
    }

    /// Pop a value from the stack
    fn pop_val(mut self) -> Result<i64, VMError> {
        if self.stack.len() > 0 {
            Ok(self.stack.pop().unwrap().into())
        } else {
            Err(VMError::StackUnderflow)
        }
    }

    /// Pop sender from the stack
    fn pop_sender(mut self) -> Result<Sender<i64>, VMError> {
        if self.stack.len() > 0 {
            match self.stack.pop().unwrap() {
                StackValue::Channel(sender, _receiver) => Ok(sender),
                _ => Err(VMError::StackUnderflow),
            }
        } else {
            Err(VMError::StackUnderflow)
        }
    }

    /// Pop receiver from the stack
    fn pop_receiver(mut self) -> Result<Receiver<i64>, VMError> {
        if self.stack.len() > 0 {
            match self.stack.pop().unwrap() {
                StackValue::Channel(_sender, receiver) => Ok(receiver),
                _ => Err(VMError::StackUnderflow),
            }
        } else {
            Err(VMError::StackUnderflow)
        }
    }

    /// Interprets the program.
    /// 
    /// Runs insructions one by one.
    pub fn interpret(mut self) -> Result<i64, VMError> {
        loop {
            let current_instruction = self.next_instruction();
            let instruction_res = match current_instruction {
                Some(instruction) => {
                    match instruction {
                        Instruction::LoadVal(val) => {
                            self.push_val(val)?;
                            None
                        },
                        Instruction::WriteVar(var_name) => {
                            println!("Varname {}", var_name);
                            let val = self.pop_val()?;
                            println!("Val {}", val);
                            self.variables.insert(var_name.to_string(), val);
                            None
                        },
                        Instruction::ReadVar(var_name) => {
                            match self.variables.get(&var_name) {
                                Some(val) => {
                                    println!("Pushing var {}", val);
                                    self.push_val(StackValue::Int(*val))?;
                                    None
                                },
                                _ => Some(VMError::StackUnderflow),
                            }
                        },
                        Instruction::FuncCall(func_name) => {
                            unimplemented!()
                        },
                        Instruction::Jump(offset) => {
                            self.ip += offset as usize;
                            None
                        },
                        Instruction::JumpBack(offset) => {
                            println!("JumpBack {}", offset);
                            self.ip -= offset as usize;
                            None
                        },
                        Instruction::JumpIfFalse(offset) => {
                            let val = self.pop_val()?;
                            println!("JumpIfFalse: {}", val);
                            if val == 0 {
                                self.ip += offset as usize;
                            }
                            None
                        },
                        Instruction::JumpIfTrue(offset) => {
                            let val = self.pop_val()?;
                            if val != 0 {
                                self.ip += offset as usize;
                            }
                            None
                        },
                        Instruction::Add => execute_native!(self, +),
                        Instruction::Sub => execute_native!(self, -),
                        Instruction::Mul => execute_native!(self, *),
                        Instruction::Div => {
                            if let (Ok(a), Ok(b)) = (self.pop_val(), self.pop_val()) {
                                if b == 0 {
                                    Some(VMError::DivisionByZero)
                                } else {
                                    self.stack.push(StackValue::Int(a / b));
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
                        Instruction::SendChannel => {
                            let value = self.pop_val()?;
                            let sender = self.pop_sender()?;
                            match sender.send(value) {
                                Ok(_) => None,
                                Err(e) => {
                                    println!("Error sending: {}", e);
                                    Some(VMError::StackUnderflow)
                                }
                            }
                        },
                        Instruction::RecvChannel => {
                            let receiver = self.pop_receiver()?;
                            let value = receiver.recv().unwrap();
                            self.push_val(StackValue::Int(value))?;
                            None
                        },
                        Instruction::Spawn => {
                            unimplemented!()
                        },
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

    // #[test]
    // fn test_arithmetic() {
    //     // Arithmetic
    //     // let x = 1
    //     // let y = 2
    //     // return (x + 1) * y
    //     let mut vm = Bytecode::new(vec![
    //         Instruction::LoadVal.into(), 0x01, 0, 0, 0, 0, 0, 0, 0,
    //         Instruction::WriteVar.into(), 0x78, 0x00, 0x00, 0x00,
    //         Instruction::LoadVal.into(), 0x02, 0, 0, 0, 0, 0, 0, 0,
    //         Instruction::WriteVar.into(), 0x79, 0x00, 0x00, 0x00,
    //         Instruction::ReadVar.into(), 0x78, 0x00, 0x00, 0x00,
    //         Instruction::LoadVal.into(), 0x01, 0, 0, 0, 0, 0, 0, 0,
    //         Instruction::Add.into(),
    //         Instruction::ReadVar.into(), 0x79, 0x00, 0x00, 0x00,
    //         Instruction::Mul.into(),
    //         Instruction::Finish.into(),
    //     ]);

    //     assert_eq!(vm.interpret().unwrap(), 4);
            
    //     // Arithmetic case 2
    //     // let x = 5
    //     // let y = 8
    //     // let z = x * y
    //     // z / 2
    //     let mut vm_1 = Bytecode::new(vec![
    //         Instruction::LoadVal.into(), 0x05, 0, 0, 0, 0, 0, 0, 0,
    //         Instruction::WriteVar.into(), 0x78, 0x00, 0x00, 0x00,
    //         Instruction::LoadVal.into(), 0x08, 0, 0, 0, 0, 0, 0, 0,
    //         Instruction::WriteVar.into(), 0x79, 0x00, 0x00, 0x00,
    //         Instruction::ReadVar.into(), 0x78, 0x00, 0x00, 0x00,
    //         Instruction::ReadVar.into(), 0x79, 0x00, 0x00, 0x00,
    //         Instruction::Mul.into(),
    //         Instruction::WriteVar.into(), 0x7A, 0x00, 0x00, 0x00,
    //         Instruction::LoadVal.into(), 0x02, 0, 0, 0, 0, 0, 0, 0,
    //         Instruction::ReadVar.into(), 0x7A, 0x00, 0x00, 0x00,
    //         Instruction::Div.into(),
    //         Instruction::Finish.into(),
    //     ]);

    //     assert_eq!(vm_1.interpret().unwrap(), 20);
    // }

    // #[test]
    // fn test_zero_division() {
    //     let mut vm = Bytecode::new(vec![
    //         Instruction::LoadVal.into(), 0x00, 0, 0, 0, 0, 0, 0, 0,
    //         Instruction::LoadVal.into(), 0x01, 0, 0, 0, 0, 0, 0, 0,
    //         Instruction::Div.into(),
    //         Instruction::Finish.into(),
    //     ]);

    //     assert_eq!(vm.interpret().unwrap_err(), VMError::DivisionByZero);
    // }

    // #[test]
    // fn test_loop() {
    //     // Pseudocode is this:
    //     //
    //     // let test = 1 + 5
    //     // while test < 10:
    //     //    test += 1
    //     // i
    //     let mut vm = Bytecode::new(vec![
    //         Instruction::LoadVal.into(), 0x01, 0, 0, 0, 0, 0, 0, 0, // 1
    //         Instruction::LoadVal.into(), 0x05, 0, 0, 0, 0, 0, 0, 0, // 5
    //         Instruction::Add.into(), // 6
    //         Instruction::WriteVar.into(), 74, 65, 73, 74, // "test"
    //         Instruction::ReadVar.into(), 74, 65, 73, 74, // "test"
    //         Instruction::LoadVal.into(), 0x0A, 0, 0, 0, 0, 0, 0, 0, // 10
    //         Instruction::Lt.into(), // test < 10
    //         Instruction::JumpIfFalse.into(), 0x16, // Skip to instruction after `JumpBack` if false
    //         Instruction::ReadVar.into(), 74, 65, 73, 74, // "test"
    //         Instruction::LoadVal.into(), 0x01, 0, 0, 0, 0, 0, 0, 0, // 1
    //         Instruction::Add.into(), // test += 1
    //         Instruction::WriteVar.into(), 74, 65, 73, 74, // "test"
    //         Instruction::JumpBack.into(), 0x27, // Jump back to loop condition
    //         Instruction::ReadVar.into(), 74, 65, 73, 74, // "test"
    //         Instruction::Finish.into(), 
    //     ]);

    //     assert_eq!(vm.interpret().unwrap(), 10);
    // }

    // #[test]
    // fn test_for_loop() {
    //     // Sum of all squares of numbers from 1 to 10
    //     // let test = 0
    //     // for temp in 1..11:
    //     //    test += temp * temp
    //     // test => 385
    //     let mut vm = Bytecode::new(vec![
    //         Instruction::LoadVal.into(), 0, 0, 0, 0, 0, 0, 0, 0,
    //         Instruction::WriteVar.into(), 0x74, 0x65, 0x73, 0x74, // "test"
    //         Instruction::LoadVal.into(), 0x01, 0, 0, 0, 0, 0, 0, 0,
    //         Instruction::WriteVar.into(), 0x74, 0x65, 0x6d, 0x70, // "temp"
    //         Instruction::LoadVal.into(), 0x0A, 0, 0, 0, 0, 0, 0, 0, // 10
    //         Instruction::ReadVar.into(), 0x74, 0x65, 0x6d, 0x70, // "temp"
    //         Instruction::Gte.into(), // temp <= 10
    //         Instruction::JumpIfFalse.into(), 44, // Skip 44 instructions forward (to after JUMP_BACK)
    //         Instruction::ReadVar.into(), 0x74, 0x65, 0x6d, 0x70,
    //         Instruction::ReadVar.into(), 0x74, 0x65, 0x6d, 0x70,
    //         Instruction::Mul.into(),
    //         Instruction::ReadVar.into(), 0x74, 0x65, 0x73, 0x74,
    //         Instruction::Add.into(),
    //         Instruction::WriteVar.into(), 0x74, 0x65, 0x73, 0x74,
    //         Instruction::ReadVar.into(), 0x74, 0x65, 0x6d, 0x70, // "temp"
    //         Instruction::LoadVal.into(), 0x01, 0, 0, 0, 0, 0, 0, 0, // 1
    //         Instruction::Add.into(), // temp++
    //         Instruction::WriteVar.into(), 0x74, 0x65, 0x6d, 0x70, // "temp"
    //         Instruction::JumpBack.into(), 61, // Jump back to the start of the loop (condition)
    //         Instruction::ReadVar.into(), 0x74, 0x65, 0x73, 0x74, 
    //         Instruction::Finish.into(),
    //     ]);

    //     assert_eq!(vm.interpret().unwrap(), 385);
    // }

    #[test]
    fn test_sender() {
        let (sender, receiver): (Sender<i64>, Receiver<i64>) = mpsc::channel();

        let mut vm = Bytecode::new(vec![
                Instruction::LoadVal(StackValue::Int(10)),
                Instruction::LoadVal(StackValue::Channel(sender, receiver)),
                Instruction::SendChannel,
                Instruction::Finish
            ]
        );

        assert_eq!(vm.interpret().unwrap(), 1);
    }
}
