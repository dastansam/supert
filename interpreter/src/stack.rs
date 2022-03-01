use std::sync::mpsc::{Sender, Receiver};

/// Type that represents a value that can be stored in the stack
#[derive(Debug)]
pub enum StackValue {
    /// Primitive value
    Int(i64),
    /// Channel
    Channel(Sender<i64>, Receiver<i64>),
}

impl Into<i64> for StackValue {
    fn into(self) -> i64 {
        match self {
            StackValue::Int(i) => i,
            StackValue::Channel(_, _) => panic!("Cannot convert channel to primitive value"),
        }
    }
}
