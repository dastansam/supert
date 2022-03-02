
use std::sync::mpsc::{Sender, Receiver};
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
    pub instructions: Vec<u8>,
    /// Program stack
    pub stack: Vec<StackValue>,
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
    pub fn new(instructions: Vec<u8>) -> Bytecode {
        Bytecode {
            instructions,
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
            self.stack.push(StackValue::Int(val));
            Ok(())
        } else {
            Err(VMError::StackOverflow)
        }
    }

    /// Pop a value from the stack
    fn pop_val(&mut self) -> Result<i64, VMError> {
        if self.stack.len() > 0 {
            Ok(self.stack.pop().unwrap().into())
        } else {
            Err(VMError::StackUnderflow)
        }
    }

    /// Pop channel
    fn pop_channel(&mut self) -> Result<(Sender<i64>, Receiver<i64>), VMError> {
        if self.stack.len() > 0 {
            match self.stack.pop().unwrap() {
                StackValue::Channel(sender, receiver) => Ok((sender, receiver)),
                _ => Err(VMError::StackUnderflow),
            }
        } else {
            Err(VMError::StackUnderflow)
        }
    }

    /// Pop sender from the stack
    /// Use this if you don't want to push the channel back onto the stack
    fn pop_sender(&mut self) -> Result<Sender<i64>, VMError> {
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
    /// Use this if you don't want to push the channel back onto the stack
    fn pop_receiver(&mut self) -> Result<Receiver<i64>, VMError> {
        if self.stack.len() > 0 {
            match self.stack.pop().unwrap() {
                StackValue::Channel(_sender, receiver) => Ok(receiver),
                _ => Err(VMError::StackUnderflow),
            }
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
                            // first two bytes are start index of the function
                            let start_ip = ((self.read_byte()? as u16) << 8) | (self.read_byte()? as u16);
                            
                            // next byte is the number of arguments
                            let num_args = self.read_byte()? as usize;

                            // read the arguments and push them onto the stack
                            for _ in 0..num_args {
                                let arg = self.read_long()?;
                                println!("Arg {}", arg);
                                self.push_val(arg)?;
                            }

                            println!("FuncCall: {} {}", start_ip, num_args);
                            self.ip = start_ip as usize;
                            None
                        },
                        Instruction::ReturnIndex => {
                            let index = ((self.read_byte()? as u16) << 8) | (self.read_byte()? as u16);
                            println!("ReturnIndex: {}", index);
                            self.ip = index as usize;
                            None
                        }
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
                            let (sender, receiver) = self.pop_channel()?;
                            match sender.send(value) {
                                Ok(_) => {},
                                Err(e) => println!("Error sending: {}", e),
                            };
                            // push the channel back onto the stack
                            // so it can be used again
                            self.stack.push(StackValue::Channel(sender, receiver));
                            None
                        },
                        Instruction::RecvChannel => {
                            let (sender, receiver) = self.pop_channel()?;
                            let value = receiver.recv().unwrap();
                            // push the channel back onto the stack
                            // so it can be used again
                            self.stack.push(StackValue::Channel(sender, receiver));
                            self.push_val(value)?;
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
            Instruction::ReadVar.into(), 0x74, 0x65, 0x6d, 0x70, // "temp"
            Instruction::LoadVal.into(), 0x0A, 0, 0, 0, 0, 0, 0, 0, // 10
            Instruction::Lte.into(), // temp <= 10
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

    #[test]
    fn test_more_loop() {
        let mut vm = Bytecode::new(vec![
            Instruction::LoadVal.into(), 0x01, 0, 0, 0, 0, 0, 0, 0, // 1
            Instruction::LoadVal.into(), 0x05, 0, 0, 0, 0, 0, 0, 0, // 5
            Instruction::Add.into(), // 6
            Instruction::WriteVar.into(), 74, 65, 73, 74, // "test"
            Instruction::ReadVar.into(), 74, 65, 73, 74, // "test"
            Instruction::LoadVal.into(), 0x64, 0, 0, 0, 0, 0, 0, 0, // 100
            Instruction::Lt.into(), // test < 100
            Instruction::JumpIfFalse.into(), 0x16, // Skip to instruction after `JumpBack` if false
            Instruction::ReadVar.into(), 74, 65, 73, 74, // "test"
            Instruction::LoadVal.into(), 0x01, 0, 0, 0, 0, 0, 0, 0, // 1
            Instruction::Add.into(), // test += 1
            Instruction::WriteVar.into(), 74, 65, 73, 74, // "test"
            Instruction::JumpBack.into(), 0x27, // Jump back to loop condition
            Instruction::ReadVar.into(), 74, 65, 73, 74, // "test"
            Instruction::Finish.into(), 
        ]);

        assert_eq!(vm.interpret().unwrap(), 100);
    }

    #[test]
    fn test_channel() {
        let instructions = vec![
            Instruction::LoadVal.into(), 0x01, 0, 0, 0, 0, 0, 0, 0,
            Instruction::SendChannel.into(),
            Instruction::RecvChannel.into(),
            Instruction::Finish.into(),
        ];

        let (sender, receiver): (Sender<i64>, Receiver<i64>) = std::sync::mpsc::channel();

        let mut vm = Bytecode {
            instructions,
            stack: vec![StackValue::Channel(sender, receiver)],
            ip: 0,
            variables: HashMap::new(),
        };

        assert_eq!(vm.interpret().unwrap(), 1);
    }

    #[test]
    fn test_func_call() {
        // fn add(x: i64, y: i64) -> i64 {
        //     x + y
        // }
        let fn_add = vec![
            Instruction::WriteVar.into(), 0x78, 0x61, 0x64, 0x64, // "xadd"
            Instruction::WriteVar.into(), 0x79, 0x61, 0x64, 0x64, // "yadd"
            Instruction::ReadVar.into(), 0x78, 0x61, 0x64, 0x64, // "xadd"
            Instruction::ReadVar.into(), 0x79, 0x61, 0x64, 0x64, // "yadd"
            Instruction::Add.into(), // xadd + yadd
        ];

        let fn_call = vec![
            Instruction::FuncCall.into(), 0x00, 0x00, 0x02, 
            0x0A, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // x: 522
            0x01, 0x01, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, // y: 65793
        ];
        
        let return_index = (fn_add.len() + fn_call.len() + 3) as u16;

        // merge fn_add and main bytecode
        // add(522, 65793)
        // => 66315
        let instructions = vec![
            fn_add.clone(),
            vec![
                Instruction::ReturnIndex.into(),
            ],
            return_index.to_le_bytes().to_vec(),
            fn_call.clone(),
            vec![
                Instruction::Finish.into(),
            ],
        ].concat();

        let mut vm = Bytecode {
            instructions,
            stack: vec![],
            ip: fn_add.len() + 3,
            variables: HashMap::new(),
        };

        assert_eq!(vm.interpret().unwrap(), 66315);
    }
}