# Design

Here I will describe some of the design choices I made during the development.

### Flatness

Each instruction is a bytecode which is followed by from 0 to 8 bytes. Bytes that follow the instruction are the data that is provided as an argument to the instruction. Here's the list of instructions and number of bytes it consumes

- `LoadVal` instruction is followed by `i64` type value, so **8 bytes**
- `Jump`, `JumpIfFalse`, `JumpIfTrue`, `JumpBack` is followed by a `u8` type (**1 byte**) which is an `offset` value, i.e number of instructions to *jump/skip*
- `WriteVar`, `ReadVar` receives **4 bytes**, i.e string with length of 4
- `Add`, `Mul`, `Div`, `Sub`, `Mod` arithmetic operations consume **0 bytes**
- `Gt`, `Gte`, `Lt`, `Lte`, `Eq`, `NotEq` comparison operators also consume **0 bytes**
- `SendChannel` consumes **8 bytes**, `RecvChannel` **0 bytes**
- `Finish` also does not consume any bytes

### StackValue

Initially, stack had the type `Vec<i64>`. But since I added the support for chanells, I had to make a type that wraps a value that can be stored in the stack. `StackValue` currently wraps `i64` and `(Sender<i64>, Receiver<i64>)` but it could easily be extended with any type.

### Channels

When adding support for channels, I had to make sure the at least one `Receiver` is open, otherwise sending value through the channel would not be supported. Therefore, both `SendChannel` and `RecvChannel` push the channel back to the stack after they are done using it.

### Improvements

One improvement to the current implementation is to make `for and while` loop implementation bit simpler. The current implementation of the loops looks roughly like this:

```md
--snip--
ReadVar 'a'
LoadVal 10
Gte
JumpIfFalse N
--snip--
/some other instructions inside the loop block/
--snip--
JumpBack M
--snip--
```

Here `N` is number of instructions to jump ahead, i.e to skip `JumpBack` instruction. And `M`, obviously, number of instructions to go back in the instructions stack. It should go back to the first instruction of condition of the loop. This makes it hard to debug when writing raw bytecode, since we would have to count the values of `N` and `M` by hand. Obviously, if we have a language with syntax and compiler for this interpreter, it would be possible to dynamically compute offset numbers while compiling.

## Answers to questions

*Describe in a few sentences how each bytecode instruction could be interpreted,
and how your interpreter or language runtime could deal with the blocking nature
of the send and the receive instructions*

Instructions related to channels: `SendChannel`, `RecvChannel`. They are interpreted similar to any other instruction and `SendChannel` consumes `i64` value to send to channel.

For `Receiver` we can use `try_recv` method to get the value in a non-blocking manner, in case buffer is empty, to push it in the top of the `instructions` list. And similarly for `Sender`, `try_send` method sends the value to the channel without blocking. In case buffer is full, we can push the instruction back to the top of the `instructions` list.

To run two methods concurrently, we can use `futures::join` macro. For that, we would need to support async methods in the interpreter.
