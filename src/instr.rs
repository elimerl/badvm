use num_enum::TryFromPrimitive;
#[repr(u8)]
#[derive(Debug, PartialEq, Eq, TryFromPrimitive)]
pub enum Instruction {
    Nop = 0x00,
    Halt,
    Push,
    Pop,
    Add,
    Mul,
    Sub,
    Div,
    Jump,
    LoadU8,
    StoreU8,
    Swap,
    Dupe,
    DupeAt,
    Interrupt,
    Call,
    Ret,
}
