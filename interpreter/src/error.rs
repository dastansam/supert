/// VM error type
#[derive(Debug, PartialEq)]
pub enum VMError {
    DivisionByZero,
    StackOverflow,
    StackUnderflow,
    UnknownInstruction,
    UnwrapError,
}
