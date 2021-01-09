use super::register::Slot;
use super::{Addr, Data};

#[derive(Debug, PartialEq, Eq)]
pub enum Opcode {
    Mov(Slot, Slot),
    Add(Slot, Slot),
    Sub(Slot, Slot),
    And(Slot, Slot),
    Or(Slot, Slot),
    Sl(Slot),
    Sr(Slot),
    Sra(Slot),
    Ldl(Slot, Data),
    Ldh(Slot, Data),
    Cmp(Slot, Slot),
    Je(Addr),
    Jmp(Addr),
    Ld(Slot, Addr),
    St(Slot, Addr),
    Hlt,
}
