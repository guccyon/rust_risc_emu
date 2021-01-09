#![allow(dead_code)]
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

#[derive(FromPrimitive, Debug)]
enum Operation {
    MOV,
    ADD,
    SUB,
    AND,
    OR,
    SL,
    SR,
    SRA,
    LDL,
    LDH,
    CMP,
    JE,
    JMP,
    LD,
    ST,
    HLT,
}

const REG0: u16 = 0;
const REG1: u16 = 1;
const REG2: u16 = 2;
const REG3: u16 = 3;

type Memory = [u16; 256];

fn assembler(rom: &mut Memory) {
    rom[0] = ldh(REG0, 0);
    rom[1] = ldl(REG0, 1);
    rom[2] = ldh(REG1, 0);
    rom[3] = ldl(REG1, 10);
    rom[4] = ldh(REG2, 0);
    rom[5] = ldl(REG2, 0);
    rom[6] = ldh(REG3, 0);
    rom[7] = ldl(REG3, 0);
    rom[8] = ldl(REG3, 0);
    rom[8] = add(REG2, REG0);
    rom[9] = add(REG3, REG2);
    rom[10] = st(REG3, 64);
    rom[11] = cmp(REG1, REG2);
    rom[12] = je(14);
    rom[13] = jmp(8);
    rom[14] = hlt();
}

fn mov(ra: u16, rb: u16) -> u16 {
    (Operation::MOV as u16) << 11 | ra << 8 | rb << 5
}
fn add(ra: u16, rb: u16) -> u16 {
    (Operation::ADD as u16) << 11 | ra << 8 | rb << 5
}
fn sub(ra: u16, rb: u16) -> u16 {
    (Operation::SUB as u16) << 11 | ra << 8 | rb << 5
}
fn and(ra: u16, rb: u16) -> u16 {
    (Operation::AND as u16) << 11 | ra << 8 | rb << 5
}
fn or(ra: u16, rb: u16) -> u16 {
    (Operation::OR as u16) << 11 | ra << 8 | rb << 5
}
fn sl(ra: u16) -> u16 {
    (Operation::SL as u16) << 11 | ra << 8
}
fn sr(ra: u16) -> u16 {
    (Operation::SR as u16) << 11 | ra << 8
}
fn sra(ra: u16) -> u16 {
    (Operation::SRA as u16) << 11 | ra << 8
}
fn ldl(ra: u16, ival: u16) -> u16 {
    (Operation::LDL as u16) << 11 | ra << 8 | ival & 0x00ff
}
fn ldh(ra: u16, ival: u16) -> u16 {
    (Operation::LDH as u16) << 11 | ra << 8 | ival & 0x00ff
}
fn cmp(ra: u16, rb: u16) -> u16 {
    (Operation::CMP as u16) << 11 | ra << 8 | rb << 5
}
fn je(addr: u16) -> u16 {
    (Operation::JE as u16) << 11 | addr & 0x00ff
}
fn jmp(addr: u16) -> u16 {
    (Operation::JMP as u16) << 11 | addr & 0x00ff
}
fn ld(_ra: u16, addr: u16) -> u16 {
    (Operation::LD as u16) << 11 | addr & 0x00ff
}
fn st(ra: u16, addr: u16) -> u16 {
    (Operation::ST as u16) << 11 | ra << 8 | addr & 0x00ff
}
fn hlt() -> u16 {
    (Operation::HLT as u16) << 11
}

fn op_code(ir: u16) -> Option<Operation> {
    FromPrimitive::from_u16(ir >> 11)
}

fn op_reg_a(ir: u16) -> usize {
    (ir >> 8 & 0b000_0000_0000_0111) as usize
}

fn op_reg_b(ir: u16) -> usize {
    (ir >> 5 & 0b000_0000_0000_0111) as usize
}

fn op_data(ir: u16) -> u16 {
    ir & 0x00ff
}

fn op_addr(ir: u16) -> usize {
    (ir & 0x00ff) as usize
}

pub fn emulate() {
    let mut rom = [0; 256];
    let mut ram = [0; 256];

    assembler(&mut rom);

    let mut pc: usize = 0;
    let mut reg = [0; 8];
    let mut flag: bool = false;

    loop {
        if rom.len() <= pc {
            break;
        }

        let ir = rom[pc];
        let op = op_code(ir).unwrap();
        pc += 1;

        use Operation::*;
        match op {
            MOV => reg[op_reg_a(ir)] = reg[op_reg_b(ir)],
            ADD => reg[op_reg_a(ir)] += reg[op_reg_b(ir)],
            SUB => reg[op_reg_a(ir)] += reg[op_reg_b(ir)],
            AND => reg[op_reg_a(ir)] = reg[op_reg_a(ir)] & reg[op_reg_b(ir)],
            OR => reg[op_reg_a(ir)] = reg[op_reg_a(ir)] | reg[op_reg_b(ir)],
            SL => reg[op_reg_a(ir)] = reg[op_reg_a(ir)] << 1,
            SR => reg[op_reg_a(ir)] = reg[op_reg_a(ir)] >> 1,
            SRA => reg[op_reg_a(ir)] = reg[op_reg_a(ir)] & 0x8000 | reg[op_reg_a(ir)] >> 1,
            LDH => reg[op_reg_a(ir)] = op_data(ir) << 8 | reg[op_reg_a(ir)] & 0x00ff,
            LDL => reg[op_reg_a(ir)] = reg[op_reg_a(ir)] & 0xff00 | op_data(ir),
            CMP => flag = reg[op_reg_a(ir)] == reg[op_reg_b(ir)],
            JE => {
                if flag {
                    pc = op_addr(ir)
                }
            }
            JMP => pc = op_addr(ir),
            LD => reg[op_reg_a(ir)] = ram[op_addr(ir)],
            ST => ram[op_addr(ir)] = reg[op_reg_a(ir)],
            HLT => break,
        }

        println!(
            "{:>3} {:04b} {:03b} {:07b} {:>3} {:>3} {:>3} {:>3}",
            pc,
            op as u16,
            ir << 5 >> 13,
            ir & 0x00ff,
            reg[0],
            reg[1],
            reg[2],
            reg[3]
        );
    }

    println!("ram[64] = {}", ram[64]);
}
