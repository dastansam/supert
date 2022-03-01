use crate::stack::StackValue;

/// Types of instructions that can be performed on the stack.
#[derive(Debug)]
pub enum Instruction {
    /// Push a value onto the stack
    LoadVal(StackValue),
    /// Write value to variable 
    /// NOTE: Next byte to this instruction is variable name and it should be one character
    WriteVar(String),
    /// Read value from a variable
    /// NOTE: Next byte to this instruction is variable name and it should be one character
    ReadVar(String),
    /// Call a method
    FuncCall(String),
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
    Jump(u8),
    /// Jumps back with the given offset
    JumpBack(u8),
    /// Jump to a specific instruction if top value on stack is true
    JumpIfTrue(u8),
    /// Jump if top value on stack is 0
    JumpIfFalse(u8),
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
    /// Jump to a specific instruction if top of stack is zero
    Finish,
}
