//
// Rust Core Imports
//
use std::fmt;

//
// Third Party Imports
//

//
// This Crate Imports
//
use errors::*;
use super::MemAddr;
use cpu::register::{Reg, reg};

#[derive(Debug, Clone, Copy)]
pub enum Opcode {
    ClearScreen,
    Return,
    SysAddr(MemAddr),
    JumpAddr(MemAddr),
    CallAddr(MemAddr),
    SkipEqByte(Reg, u8),
    SkipNEqByte(Reg, u8),
    SkipEqReg(Reg, Reg),
    LoadByte(Reg, u8),
    AddByte(Reg, u8),
    LoadReg(Reg, Reg),
    BitOr(Reg, Reg),
    BitAnd(Reg, Reg),
    BitXor(Reg, Reg),
    MathAdd(Reg, Reg),
    MathSub(Reg, Reg),
    ShiftRight(Reg, Reg),
    MathSubN(Reg, Reg),
    ShiftLeft(Reg, Reg),
    SkipNEqReg(Reg, Reg),
    MemLoad(MemAddr),
    JumpAddV0(MemAddr),
    Rand(Reg, u8),
    Draw(Reg, Reg, u8),
    KeyEqSkip(Reg),
    KeyNEqSkip(Reg),
    DelayGet(Reg),
    KeyGet(Reg),
    DelaySet(Reg),
    SoundSet(Reg),
    MemAdd(Reg),
    MemSprite(Reg),
    BCD(Reg),
    RegDump(Reg),
    RegLoad(Reg),
}

#[inline]
fn halfword2nibbles(x: u16) -> (u8, u8, u8, u8) {
    let a = ((x >> 0xC) & 0x0F) as u8;
    let b = ((x >> 0x8) & 0x0F) as u8;
    let c = ((x >> 0x4) & 0x0F) as u8;
    let d = ((x) & 0x0F) as u8;
    (a, b, c, d)
}

#[inline]
fn nibble2byte(a: u8, b: u8) -> u8 {
    (a << 4) | b & 0x0F
}

#[inline]
fn nibbles2addr(a: u8, b: u8, c: u8) -> MemAddr {
    let mut x = a as MemAddr;
    x = x << 8;
    x = x | nibble2byte(b, c) as MemAddr;
    x
}

pub fn disassemble(instr: u16) -> Result<Opcode> {

    match halfword2nibbles(instr) {
        (0x0, 0, 0xE, 0) => Ok(Opcode::ClearScreen),
        (0x0, 0, 0xE, 0xE) => Ok(Opcode::Return),
        (0x0, n1, n2, n3) => Ok(Opcode::SysAddr(nibbles2addr(n1, n2, n3))),
        (0x1, n1, n2, n3) => Ok(Opcode::JumpAddr(nibbles2addr(n1, n2, n3))),
        (0x2, n1, n2, n3) => Ok(Opcode::CallAddr(nibbles2addr(n1, n2, n3))),
        (0x3, x, n1, n2) => Ok(Opcode::SkipEqByte(reg(x), nibble2byte(n1, n2))),
        (0x4, x, n1, n2) => Ok(Opcode::SkipNEqByte(reg(x), nibble2byte(n1, n2))),
        (0x5, x, y, 0) => Ok(Opcode::SkipEqReg(reg(x), reg(y))),
        (0x6, x, n1, n2) => Ok(Opcode::LoadByte(reg(x), nibble2byte(n1, n2))),
        (0x7, x, n1, n2) => Ok(Opcode::AddByte(reg(x), nibble2byte(n1, n2))),
        (0x8, x, y, 0) => Ok(Opcode::LoadReg(reg(x), reg(y))),
        (0x8, x, y, 1) => Ok(Opcode::BitOr(reg(x), reg(y))),
        (0x8, x, y, 2) => Ok(Opcode::BitAnd(reg(x), reg(y))),
        (0x8, x, y, 3) => Ok(Opcode::BitXor(reg(x), reg(y))),
        (0x8, x, y, 4) => Ok(Opcode::MathAdd(reg(x), reg(y))),
        (0x8, x, y, 5) => Ok(Opcode::MathSub(reg(x), reg(y))),
        (0x8, x, y, 6) => Ok(Opcode::ShiftRight(reg(x), reg(y))),
        (0x8, x, y, 7) => Ok(Opcode::MathSubN(reg(x), reg(y))),
        (0x8, x, y, 0xE) => Ok(Opcode::ShiftLeft(reg(x), reg(y))),
        (0x9, x, y, 0) => Ok(Opcode::SkipNEqReg(reg(x), reg(y))),
        (0xA, n1, n2, n3) => Ok(Opcode::MemLoad(nibbles2addr(n1, n2, n3))),
        (0xB, n1, n2, n3) => Ok(Opcode::JumpAddV0(nibbles2addr(n1, n2, n3))),
        (0xC, x, n1, n2) => Ok(Opcode::Rand(reg(x), nibble2byte(n1, n2))),
        (0xD, x, y, n1) => Ok(Opcode::Draw(reg(x), reg(y), n1)),
        (0xE, x, 0x9, 0xE) => Ok(Opcode::KeyEqSkip(reg(x))),
        (0xE, x, 0xA, 0x1) => Ok(Opcode::KeyNEqSkip(reg(x))),
        (0xF, x, 0x0, 0x7) => Ok(Opcode::DelayGet(reg(x))),
        (0xF, x, 0x0, 0xA) => Ok(Opcode::KeyGet(reg(x))),
        (0xF, x, 0x1, 0x5) => Ok(Opcode::DelaySet(reg(x))),
        (0xF, x, 0x1, 0x8) => Ok(Opcode::SoundSet(reg(x))),
        (0xF, x, 0x1, 0xE) => Ok(Opcode::MemAdd(reg(x))),
        (0xF, x, 0x2, 0x9) => Ok(Opcode::MemSprite(reg(x))),
        (0xF, x, 0x3, 0x3) => Ok(Opcode::BCD(reg(x))),
        (0xF, x, 0x5, 0x5) => Ok(Opcode::RegDump(reg(x))),
        (0xF, x, 0x6, 0x5) => Ok(Opcode::RegLoad(reg(x))),
        _ => bail!(ErrorKind::UnrecognizedOpcode(instr)),
    }
}
impl fmt::Display for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Opcode::ClearScreen => write!(f, "CLS"),
            &Opcode::Return => write!(f, "RETURN"),
            &Opcode::SysAddr(addr) => write!(f, "SYSADDR 0x{:04x}", addr),
            &Opcode::JumpAddr(addr) => write!(f, "JUMP 0x{:04x}", addr),
            &Opcode::CallAddr(addr) => write!(f, "CALL 0x{:04x}", addr),
            &Opcode::SkipEqByte(x, byte) => write!(f, "SKIP.EQ {} 0x{:02x}", x, byte),
            &Opcode::SkipNEqByte(x, byte) => write!(f, "SKIP.NEQ {} 0x{:02x}", x, byte),
            &Opcode::SkipEqReg(x, y) => write!(f, "SKIP.EQ {} {}", x, y),
            &Opcode::LoadByte(x, byte) => write!(f, "LOAD {} 0x{:02x}", x, byte),
            &Opcode::AddByte(x, byte) => write!(f, "MATH.ADD {} 0x{:02x}", x, byte),
            &Opcode::LoadReg(x, y) => write!(f, "LOAD {} {}", x, y),
            &Opcode::BitOr(x, y) => write!(f, "BIT.OR {} {}", x, y),
            &Opcode::BitAnd(x, y) => write!(f, "BIT.AND {} {}", x, y),
            &Opcode::BitXor(x, y) => write!(f, "BIT.XOR {} {}", x, y),
            &Opcode::MathAdd(x, y) => write!(f, "MATH.ADD {} {}", x, y),
            &Opcode::MathSub(x, y) => write!(f, "MATH.SUB {} {}", x, y),
            &Opcode::ShiftRight(x, y) => write!(f, "BIT.SHR {} {}", x, y),
            &Opcode::MathSubN(x, y) => write!(f, "MATH.SUBN {} {}", x, y),
            &Opcode::ShiftLeft(x, y) => write!(f, "BIT.SHL {} {}", x, y),
            &Opcode::SkipNEqReg(x, y) => write!(f, "SKIP.NEQ {} {}", x, y),
            &Opcode::MemLoad(addr) => write!(f, "LOAD VI 0x{:04x}", addr),
            &Opcode::JumpAddV0(addr) => write!(f, "LOAD VI V0+0x{:04x}", addr),
            &Opcode::Rand(x, byte) => write!(f, "RAND {} 0x{:02x}", x, byte),
            &Opcode::Draw(x, y, byte) => write!(f, "DRAW {} {} 0x{:02x}", x, y, byte),
            &Opcode::KeyEqSkip(x) => write!(f, "SKIP.KEY {}", x),
            &Opcode::KeyNEqSkip(x) => write!(f, "SKIP.NKEY {}", x),
            &Opcode::DelayGet(x) => write!(f, "DELAY.GET {}", x),
            &Opcode::KeyGet(x) => write!(f, "KEY.GET {}", x),
            &Opcode::DelaySet(x) => write!(f, "DELAY.SET {}", x),
            &Opcode::SoundSet(x) => write!(f, "SOUND.SET {}", x),
            &Opcode::MemAdd(x) => write!(f, "MATH.ADD VI {}", x),
            &Opcode::MemSprite(x) => write!(f, "SPRITE {}", x),
            &Opcode::BCD(x) => write!(f, "BCD {}", x),
            &Opcode::RegDump(x) => write!(f, "REG.DUMP {}", x),
            &Opcode::RegLoad(x) => write!(f, "REG.LOAD {}", x),
        }
    }
}

#[test]
fn halfwordsplit() {
    assert_eq!((0x0, 0x0, 0x0, 0x0), halfword2nibbles(0x0000));
    assert_eq!((0x1, 0x0, 0x1, 0x0), halfword2nibbles(0x1010));
    assert_eq!((0x1, 0x1, 0x0, 0x1), halfword2nibbles(0x1101));
    assert_eq!((0x8, 0x3, 0x7, 0x0), halfword2nibbles(0x8370));
    assert_eq!((0xA, 0x3, 0x3, 0xB), halfword2nibbles(0xA33B));
    assert_eq!((0x4, 0xE, 0x9, 0xB), halfword2nibbles(0x4E9B));
    assert_eq!((0xF, 0xF, 0xF, 0xF), halfword2nibbles(0xFFFF));
}

#[test]
fn bytejoin() {
    assert_eq!(nibble2byte(0x0, 0x0), 0x00);
    assert_eq!(nibble2byte(0x1, 0x0), 0x10);
    assert_eq!(nibble2byte(0x1, 0x1), 0x11);
    assert_eq!(nibble2byte(0x8, 0x3), 0x83);
    assert_eq!(nibble2byte(0xA, 0x3), 0xA3);
    assert_eq!(nibble2byte(0x4, 0xE), 0x4E);
    assert_eq!(nibble2byte(0xF, 0xF), 0xFF);

    // Try to mess it up
    assert_eq!(nibble2byte(0x10, 0x60), 0x00);
    assert_eq!(nibble2byte(0x41, 0x60), 0x10);
    assert_eq!(nibble2byte(0xA1, 0x91), 0x11);
    assert_eq!(nibble2byte(0x98, 0xA3), 0x83);
    assert_eq!(nibble2byte(0x2A, 0xF3), 0xA3);
    assert_eq!(nibble2byte(0x34, 0x1E), 0x4E);
    assert_eq!(nibble2byte(0x4F, 0x6F), 0xFF);

}

#[test]
fn addr_from_nibbles() {
    assert_eq!(nibbles2addr(0x0, 0x0, 0x0), 0x0000);
    assert_eq!(nibbles2addr(0x1, 0x0, 0x0), 0x0100);
    assert_eq!(nibbles2addr(0x1, 0x1, 0x7), 0x0117);
    assert_eq!(nibbles2addr(0x8, 0x3, 0xE), 0x083E);
    assert_eq!(nibbles2addr(0xA, 0x3, 0xD), 0x0A3D);
    assert_eq!(nibbles2addr(0x4, 0xE, 0xB), 0x04EB);
    assert_eq!(nibbles2addr(0xF, 0xF, 0xF), 0x0FFF);

}
