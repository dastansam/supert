/// Types of instructions that can be performed on the stack.
#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
    /// Push a value onto the stack
    LoadVal,
    /// Write value to variable 
    WriteVar,
    /// Read value from a variable
    ReadVar,
    /// Call a method
    FuncCall,
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
    /// Next byte is the offset
    Jump,
    /// Jumps back with the given offset
    JumpBack,
    /// Jump to a specific instruction if top value on stack is true
    JumpIfTrue,
    /// Jump if top value on stack is 0
    JumpIfFalse,
    /// Not equal
    NotEq,
    /// Equal
    Eq,
    /// Greater than
    Gt,
    /// Less than
    Lt,
    /// Greater than or equal to
    Gte,
    /// Less than or equal to
    Lte,
    /// Pops the channel and a value from the stack and sends the value to the channel using a blocking send
    SendChannel,
    /// Pops the channel from the stack, receives a value from the channel (this may block) and pushes it onto the stack
    RecvChannel,
    /// Pops the channel from the stack and closes the channel
    Spawn,
    /// Returns return index of the function
    ReturnIndex,
    /// Jump to a specific instruction if top of stack is zero
    Finish,
}

impl From<u8> for Instruction {
    fn from(byte: u8) -> Self {
        match byte {
            0 => Instruction::LoadVal,
            1 => Instruction::WriteVar,
            2 => Instruction::ReadVar,
            3 => Instruction::FuncCall,
            4 => Instruction::Add,
            5 => Instruction::Sub,
            6 => Instruction::Mul,
            7 => Instruction::Div,
            8 => Instruction::Mod,
            9 => Instruction::Jump,
            10 => Instruction::JumpBack,
            11 => Instruction::JumpIfTrue,
            12 => Instruction::JumpIfFalse,
            13 => Instruction::NotEq,
            14 => Instruction::Eq,
            15 => Instruction::Gt,
            16 => Instruction::Lt,
            17 => Instruction::Gte,
            18 => Instruction::Lte,
            19 => Instruction::SendChannel,
            20 => Instruction::RecvChannel,
            21 => Instruction::Spawn,
            22 => Instruction::ReturnIndex,
            23 => Instruction::Finish,
            _ => panic!("Invalid instruction byte: {}", byte),
        }
    }
}

impl Into<u8> for Instruction {
    fn into(self) -> u8 {
        match self {
            Instruction::LoadVal => 0,
            Instruction::WriteVar => 1,
            Instruction::ReadVar => 2,
            Instruction::FuncCall => 3,
            Instruction::Add => 4,
            Instruction::Sub => 5,
            Instruction::Mul => 6,
            Instruction::Div => 7,
            Instruction::Mod => 8,
            Instruction::Jump => 9,
            Instruction::JumpBack => 10,
            Instruction::JumpIfTrue => 11,
            Instruction::JumpIfFalse => 12,
            Instruction::NotEq => 13,
            Instruction::Eq => 14,
            Instruction::Gt => 15,
            Instruction::Lt => 16,
            Instruction::Gte => 17,
            Instruction::Lte => 18,
            Instruction::SendChannel => 19,
            Instruction::RecvChannel => 20,
            Instruction::Spawn => 21,
            Instruction::ReturnIndex => 22,
            Instruction::Finish => 23,
        }
    }
}
