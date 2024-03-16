//! Defines a simple for loop example
use general_assembly::{condition::Condition, operand::Operand, operand::DataWord, operation::Operation};
use transpiler::pseudo;

pub trait LocalInto<T> {
    fn local_into(self) -> T;
}

impl LocalInto<Operand>  for u32 {
    fn local_into(self) -> Operand{
        Operand::Immidiate(DataWord::Word32(self))
    }
}

fn main() {
    let a = Operand::Register("a".to_owned());
    let b = Operand::Register("b".to_owned());
    let c = Operand::Local("c".to_owned());
    let regs = [a,b];
    let cond = false;
    let ret = pseudo!([
        for reg in regs.into_iter() {
            reg = reg + 1u32.local_into();
        }
    ]);
}

