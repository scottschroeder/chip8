//
// Rust Core Imports
//

//
// Third Party Imports
//

//
// This Crate Imports
//
use errors::*;
use super::MemAddr;
use cpu::register::{Reg, reg};

#[derive(Debug)]
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
    KeyEqJump(Reg),
    KeyNEqJump(Reg),
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
fn byte2nibble(x: u8) -> (u8, u8) {
    let a = (x >> 4) & 0x0F;
    let b = (x) & 0x0F;
    (a, b)
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

pub fn disassemble(b1: u8, b2: u8) -> Result<Opcode> {

    let (a, b) = byte2nibble(b1);
    let (c, d) = byte2nibble(b2);

    match (a, b, c, d) {
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
        (0xE, x, 0x9, 0xE) => Ok(Opcode::KeyEqJump(reg(x))),
        (0xE, x, 0xA, 0x1) => Ok(Opcode::KeyNEqJump(reg(x))),
        (0xF, x, 0x0, 0x7) => Ok(Opcode::DelayGet(reg(x))),
        (0xF, x, 0x0, 0xA) => Ok(Opcode::KeyGet(reg(x))),
        (0xF, x, 0x1, 0x5) => Ok(Opcode::DelaySet(reg(x))),
        (0xF, x, 0x1, 0x8) => Ok(Opcode::SoundSet(reg(x))),
        (0xF, x, 0x1, 0xE) => Ok(Opcode::MemAdd(reg(x))),
        (0xF, x, 0x2, 0x9) => Ok(Opcode::MemSprite(reg(x))),
        (0xF, x, 0x3, 0x3) => Ok(Opcode::BCD(reg(x))),
        (0xF, x, 0x5, 0x5) => Ok(Opcode::RegDump(reg(x))),
        (0xF, x, 0x6, 0x5) => Ok(Opcode::RegLoad(reg(x))),
        _ => bail!(ErrorKind::UnrecognizedOpcode(b1, b2)),
    }
}

#[test]
fn bytesplit() {
    assert_eq!((0x0, 0x0), byte2nibble(0x00));
    assert_eq!((0x1, 0x0), byte2nibble(0x10));
    assert_eq!((0x1, 0x1), byte2nibble(0x11));
    assert_eq!((0x8, 0x3), byte2nibble(0x83));
    assert_eq!((0xA, 0x3), byte2nibble(0xA3));
    assert_eq!((0x4, 0xE), byte2nibble(0x4E));
    assert_eq!((0xF, 0xF), byte2nibble(0xFF));
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
fn byte_split_join() {
    for i in 0..0xFF {
        let byte = i as u8;
        let (n1, n2) = byte2nibble(byte);
        let re_byte = nibble2byte(n1, n2);
        assert_eq!(byte, re_byte);
    }
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
