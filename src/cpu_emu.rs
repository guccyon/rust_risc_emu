mod ir;
mod opcode;
mod register;
mod rom;

use ir::InstructionRegister;
use opcode::Opcode;
use register::{GeneralRegister, Slot};
pub use rom::Rom;

type Addr = usize;
type Data = u16;

#[derive(Debug)]
pub struct CpuEmu {
    pc: usize,
    ir: InstructionRegister,
    register: GeneralRegister,
    flag: bool,
    rom: Rom,
    ram: [u16; 256],
}

impl CpuEmu {
    pub fn new(rom: Rom) -> Self {
        Self {
            register: GeneralRegister::new(),
            ir: InstructionRegister::new(),
            pc: 0,
            flag: false,
            rom: rom,
            ram: [0; 256],
        }
    }

    pub fn run(&mut self) -> Result<(), String> {
        loop {
            self.fetch()?;

            match self.decode()? {
                Opcode::Hlt => break Ok(()),
                code => self.execute(code)?,
            }
        }
    }

    fn fetch(&mut self) -> Result<(), String> {
        self.ir.write(self.rom.read(self.pc)?);
        self.pc += 1;
        Ok(())
    }

    fn decode(&self) -> Result<Opcode, String> {
        use Opcode::*;

        let code = match self.ir.code() {
            0b0000 => Mov(self.ir.reg_a(), self.ir.reg_b()),
            0b0001 => Add(self.ir.reg_a(), self.ir.reg_b()),
            0b0010 => Sub(self.ir.reg_a(), self.ir.reg_b()),
            0b0011 => And(self.ir.reg_a(), self.ir.reg_b()),
            0b0100 => Or(self.ir.reg_a(), self.ir.reg_b()),
            0b0101 => Sl(self.ir.reg_a()),
            0b0110 => Sr(self.ir.reg_a()),
            0b0111 => Sra(self.ir.reg_a()),
            0b1000 => Ldl(self.ir.reg_a(), self.ir.data()),
            0b1001 => Ldh(self.ir.reg_a(), self.ir.data()),
            0b1010 => Cmp(self.ir.reg_a(), self.ir.reg_b()),
            0b1011 => Je(self.ir.addr()),
            0b1100 => Jmp(self.ir.addr()),
            0b1101 => Ld(self.ir.reg_a(), self.ir.addr()),
            0b1110 => St(self.ir.reg_a(), self.ir.addr()),
            0b1111 => Hlt,
            _ => return Err("unknown operation code".to_string()),
        };

        Ok(code)
    }

    fn execute(&mut self, code: Opcode) -> Result<(), String> {
        use opcode::Opcode::*;

        match code {
            Mov(reg_a, reg_b) => self.register.write(reg_a, self.register.read(reg_b)),
            Add(reg_a, reg_b) => {
                let data = self.register.read(reg_a) + self.register.read(reg_b);
                self.register.write(reg_a, data)
            }
            Sub(reg_a, reg_b) => {
                let data = self.register.read(reg_a) - self.register.read(reg_b);
                self.register.write(reg_a, data)
            }
            And(reg_a, reg_b) => {
                let data = self.register.read(reg_a) & self.register.read(reg_b);
                self.register.write(reg_a, data)
            }
            Or(reg_a, reg_b) => {
                let data = self.register.read(reg_a) | self.register.read(reg_b);
                self.register.write(reg_a, data)
            }
            Sl(reg_a) => self.register.write(reg_a, self.register.read(reg_a) << 1),
            Sr(reg_a) => self.register.write(reg_a, self.register.read(reg_a) >> 1),
            Sra(reg_a) => {
                let data1 = self.register.read(reg_a) & 0b1000_0000_0000_0000;
                let data2 = self.register.read(reg_a) >> 1;
                self.register.write(reg_a, data1 | data2)
            }
            Ldl(reg_a, data) => {
                let high = self.register.read(reg_a) & 0xff00;
                let low = data & 0x00ff;
                self.register.write(reg_a, high | low)
            }
            Ldh(reg_a, data) => {
                let high = data << 8 & 0xff00;
                let low = self.register.read(reg_a) & 0x00ff;
                self.register.write(reg_a, high | low)
            }
            Cmp(reg_a, reg_b) => self.flag = self.register.read(reg_a) == self.register.read(reg_b),
            Je(addr) => {
                if self.flag {
                    self.pc = addr
                }
            }
            Jmp(addr) => self.pc = addr,
            Ld(reg_a, addr) => self.register.write(reg_a, self.ram[addr]),
            St(reg_a, addr) => self.ram[addr] = self.register.read(reg_a),
            _ => {}
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn halt() -> u16 {
        0b1111_000_000_00000
    }

    #[test]
    fn test_halt() {
        if let Err(msg) = CpuEmu::new(Rom::new(vec![halt()])).run() {
            panic!(msg);
        }

        assert!(true);
    }

    #[test]
    fn test_run_mov() {
        let rom = Rom::new(vec![0b0000_000_001_00000, halt()]);
        let mut cpu = CpuEmu::new(rom);
        cpu.register.write(Slot::Reg1, 30);
        assert_eq!(cpu.register.read(Slot::Reg1), 30);
        assert_eq!(cpu.register.read(Slot::Reg0), 0);

        if let Err(msg) = cpu.run() {
            panic!(msg);
        }

        assert_eq!(cpu.register.read(Slot::Reg0), 30)
    }

    #[test]
    fn test_run_add() {
        let rom = Rom::new(vec![0b0001_000_001_00000, halt()]);
        let mut cpu = CpuEmu::new(rom);
        cpu.register.write(Slot::Reg0, 5);
        cpu.register.write(Slot::Reg1, 15);

        assert_eq!(cpu.register.read(Slot::Reg0), 5);
        assert_eq!(cpu.register.read(Slot::Reg1), 15);

        if let Err(msg) = cpu.run() {
            panic!(msg);
        }

        assert_eq!(cpu.register.read(Slot::Reg0), 20);
        assert_eq!(cpu.register.read(Slot::Reg1), 15);
    }

    #[test]
    fn test_run_sub() {
        let rom = Rom::new(vec![0b0010_000_001_00000, halt()]);
        let mut cpu = CpuEmu::new(rom);
        cpu.register.write(Slot::Reg0, 15);
        cpu.register.write(Slot::Reg1, 5);

        assert_eq!(cpu.register.read(Slot::Reg0), 15);

        if let Err(msg) = cpu.run() {
            panic!(msg);
        }

        assert_eq!(cpu.register.read(Slot::Reg0), 10);
    }

    #[test]
    fn test_run_and() {
        let rom = Rom::new(vec![0b0011_000_001_00000, halt()]);
        let mut cpu = CpuEmu::new(rom);
        cpu.register.write(Slot::Reg0, 0b0101);
        cpu.register.write(Slot::Reg1, 0b0110);

        assert_eq!(cpu.register.read(Slot::Reg0), 0b0101);

        if let Err(msg) = cpu.run() {
            panic!(msg);
        }

        assert_eq!(cpu.register.read(Slot::Reg0), 0b0100);
    }

    #[test]
    fn test_run_or() {
        let rom = Rom::new(vec![0b0100_000_001_00000, halt()]);
        let mut cpu = CpuEmu::new(rom);
        cpu.register.write(Slot::Reg0, 0b0101);
        cpu.register.write(Slot::Reg1, 0b0110);

        assert_eq!(cpu.register.read(Slot::Reg0), 0b0101);

        if let Err(msg) = cpu.run() {
            panic!(msg);
        }

        assert_eq!(cpu.register.read(Slot::Reg0), 0b0111);
    }

    #[test]
    fn test_run_sl() {
        let rom = Rom::new(vec![0b0101_001_000_00000, halt()]);
        let mut cpu = CpuEmu::new(rom);
        cpu.register.write(Slot::Reg1, 0b0011);

        assert_eq!(cpu.register.read(Slot::Reg1), 0b0011);

        if let Err(msg) = cpu.run() {
            panic!(msg);
        }

        assert_eq!(cpu.register.read(Slot::Reg1), 0b0110);
    }

    #[test]
    fn test_run_sr() {
        let rom = Rom::new(vec![0b0110_001_000_00000, halt()]);
        let mut cpu = CpuEmu::new(rom);
        cpu.register.write(Slot::Reg1, 0b1100);

        assert_eq!(cpu.register.read(Slot::Reg1), 0b1100);

        if let Err(msg) = cpu.run() {
            panic!(msg);
        }

        assert_eq!(cpu.register.read(Slot::Reg1), 0b0110);
    }

    #[test]
    fn test_run_sra() {
        let rom = Rom::new(vec![0b0111_000_000_00000, 0b0111_001_000_00000, halt()]);
        let mut cpu = CpuEmu::new(rom);
        cpu.register.write(Slot::Reg0, 0b0000_0000_1100_0001);
        cpu.register.write(Slot::Reg1, 0b1000_0000_1100_0001);

        assert_eq!(cpu.register.read(Slot::Reg0), 0b0000_0000_1100_0001);
        assert_eq!(cpu.register.read(Slot::Reg1), 0b1000_0000_1100_0001);

        if let Err(msg) = cpu.run() {
            panic!(msg);
        }

        assert_eq!(cpu.register.read(Slot::Reg0), 0b0000_0000_0110_0000);
        assert_eq!(cpu.register.read(Slot::Reg1), 0b1100_0000_0110_0000);
    }

    #[test]
    fn test_run_ldl() {
        let rom = Rom::new(vec![0b1000_000_10110010, halt()]);
        let mut cpu = CpuEmu::new(rom);
        cpu.register.write(Slot::Reg0, 0b0110_0000_0000_0000);

        assert_eq!(cpu.register.read(Slot::Reg0), 0b0110_0000_0000_0000);

        if let Err(msg) = cpu.run() {
            panic!(msg);
        }

        assert_eq!(cpu.register.read(Slot::Reg0), 0b0110_0000_10110010);
    }

    #[test]
    fn test_run_ldh() {
        let rom = Rom::new(vec![0b1001_000_10110010, halt()]);
        let mut cpu = CpuEmu::new(rom);
        cpu.register.write(Slot::Reg0, 0b0000_0000_0000_0101);

        assert_eq!(cpu.register.read(Slot::Reg0), 0b00000000_00000101);

        if let Err(msg) = cpu.run() {
            panic!(msg);
        }

        assert_eq!(cpu.register.read(Slot::Reg0), 0b10110010_00000101);
    }

    #[test]
    fn test_run_cmp_true() {
        let rom = Rom::new(vec![0b1010_000_001_00000, halt()]);
        let mut cpu = CpuEmu::new(rom);
        cpu.register.write(Slot::Reg0, 5);
        cpu.register.write(Slot::Reg1, 5);

        assert_eq!(cpu.flag, false);
        if let Err(msg) = cpu.run() {
            panic!(msg);
        }
        assert_eq!(cpu.flag, true);
    }

    #[test]
    fn test_run_cmp_false() {
        let rom = Rom::new(vec![0b1010_000_001_00000, halt()]);
        let mut cpu = CpuEmu::new(rom);
        cpu.register.write(Slot::Reg0, 5);
        cpu.register.write(Slot::Reg1, 6);

        assert_eq!(cpu.flag, false);
        if let Err(msg) = cpu.run() {
            panic!(msg);
        }
        assert_eq!(cpu.flag, false);
    }

    #[test]
    fn test_run_je() {
        let instructions = vec![
            0b1010_001_010_00000, // cmp reg1, reg2 => true
            0b1011_000_00000011,  // jump to halt() => 3
            0b0101_000_00000000,  // shift left
            halt(),
        ];
        let rom = Rom::new(instructions);
        let mut cpu = CpuEmu::new(rom);
        cpu.register.write(Slot::Reg0, 10);

        assert!(cpu.register.read(Slot::Reg1) == cpu.register.read(Slot::Reg2));
        assert_eq!(cpu.register.read(Slot::Reg0), 10);

        if let Err(msg) = cpu.run() {
            panic!(msg);
        }
        assert_eq!(cpu.register.read(Slot::Reg0), 10);
    }

    #[test]
    fn test_run_jmp() {
        let instructions = vec![
            0b1100_000_00000010, // jump to halt() => 2
            0b0101_000_00000000, // shift left
            halt(),
        ];
        let rom = Rom::new(instructions);
        let mut cpu = CpuEmu::new(rom);
        cpu.register.write(Slot::Reg0, 10);

        assert_eq!(cpu.flag, false);
        assert_eq!(cpu.register.read(Slot::Reg0), 10);

        if let Err(msg) = cpu.run() {
            panic!(msg);
        }
        assert_eq!(cpu.register.read(Slot::Reg0), 10);
    }

    #[test]
    fn test_run_ld() {
        let rom = Rom::new(vec![0b1101_000_00000111, halt()]);
        let mut cpu = CpuEmu::new(rom);
        cpu.ram[7] = 100;

        assert_eq!(cpu.register.read(Slot::Reg0), 0);
        assert_eq!(cpu.ram[7], 100);
        if let Err(msg) = cpu.run() {
            panic!(msg);
        }
        assert_eq!(cpu.register.read(Slot::Reg0), 100);
    }

    #[test]
    fn test_run_st() {
        let rom = Rom::new(vec![0b1110_000_00000111, halt()]);
        let mut cpu = CpuEmu::new(rom);
        cpu.register.write(Slot::Reg0, 50);

        assert_eq!(cpu.register.read(Slot::Reg0), 50);
        assert_eq!(cpu.ram[7], 0);
        if let Err(msg) = cpu.run() {
            panic!(msg);
        }
        assert_eq!(cpu.ram[7], 50);
    }
}
