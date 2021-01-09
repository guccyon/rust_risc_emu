use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

#[derive(Debug, Copy, Clone, PartialEq, Eq, FromPrimitive)]
pub enum Slot {
    Reg0,
    Reg1,
    Reg2,
    Reg3,
    Reg4,
    Reg5,
    Reg6,
    Reg7,
}

impl From<u16> for Slot {
    fn from(from: u16) -> Slot {
        FromPrimitive::from_u16(from).unwrap()
    }
}

#[derive(Debug)]
pub struct GeneralRegister {
    regs: [u16; 8],
}

impl GeneralRegister {
    pub fn new() -> Self {
        Self { regs: [0; 8] }
    }

    pub fn read(&self, slot: Slot) -> u16 {
        self.regs[slot as usize]
    }

    pub fn write(&mut self, slot: Slot, data: u16) {
        self.regs[slot as usize] = data;
    }
}

#[cfg(test)]
mod tests {
    use super::GeneralRegister;
    use super::Slot::*;

    #[test]
    fn test_write_read() {
        let mut register = GeneralRegister::new();

        assert_eq!(register.read(Reg0), 0);
        register.write(Reg0, 10);
        assert_eq!(register.read(Reg0), 10);
    }

    #[test]
    fn test_write_read2() {
        let mut register = GeneralRegister::new();

        assert_eq!(register.read(Reg3), 0);
        register.write(Reg3, 20);
        assert_eq!(register.read(Reg3), 20);
    }
}
