/// - No, just interpreter.
/// - If you can manage functions and inputs, yes.
/// - Flat is as a single enum without nested enums, keep it simple.
mod vm;
mod instruction;
mod error;
mod stack;

use instruction::Instruction;
use vm::Bytecode;

pub fn main() {
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

    println!("{:?}", vm.interpret().unwrap());
}
