use super::register::Slot;
use super::{Addr, Data};

#[derive(Debug)]
pub struct InstructionRegister {
    instruction: u16,
}

impl InstructionRegister {
    pub fn new() -> Self {
        Self { instruction: 0 }
    }

    pub fn write(&mut self, instruction: u16) {
        self.instruction = instruction;
    }

    pub fn code(&self) -> u16 {
        self.instruction >> 11
    }

    pub fn reg_a(&self) -> Slot {
        let index = self.instruction >> 8 & 0x0007; // 0 to 7
        index.into()
    }

    pub fn reg_b(&self) -> Slot {
        let index = self.instruction >> 5 & 0x0007; // 0 to 7
        index.into()
    }

    pub fn data(&self) -> Data {
        self.instruction & 0x00ff
    }

    pub fn addr(&self) -> Addr {
        (self.instruction & 0x00ff) as Addr
    }
}

#[cfg(test)]
mod tests {
    use super::super::register::Slot;
    use super::super::*;

    #[test]
    fn test_reg_a_1() {
        let mut register = InstructionRegister::new();
        register.write(0b0000_000_000_00000);
        assert_eq!(register.reg_a(), Slot::Reg0);
    }

    #[test]
    fn test_reg_a_2() {
        let mut register = InstructionRegister::new();
        register.write(0b0000_111_000_00000);
        assert_eq!(register.reg_a(), Slot::Reg7);
    }

    #[test]
    fn test_reg_b_1() {
        let mut register = InstructionRegister::new();
        register.write(0b0000_000_001_00000);
        assert_eq!(register.reg_b(), Slot::Reg1);
    }

    #[test]
    fn test_reg_b_2() {
        let mut register = InstructionRegister::new();
        register.write(0b0000_000_011_00000);
        assert_eq!(register.reg_b(), Slot::Reg3);
    }

    #[test]
    fn test_data_1() {
        let mut register = InstructionRegister::new();
        register.write(15);
        assert_eq!(register.data(), 15);
    }

    #[test]
    fn test_data_2() {
        let mut register = InstructionRegister::new();
        register.write(0b1001_000_000_01111);
        assert_eq!(register.data(), 15);
    }

    #[test]
    fn test_addr_1() {
        let mut register = InstructionRegister::new();
        register.write(15);
        assert_eq!(register.addr(), 15);
    }

    #[test]
    fn test_addr_2() {
        let mut register = InstructionRegister::new();
        register.write(0b1001_000_000_01111);
        assert_eq!(register.addr(), 15);
    }
}
