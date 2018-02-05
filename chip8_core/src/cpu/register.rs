use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum Reg {
    V0 = 0x0,
    V1 = 0x1,
    V2 = 0x2,
    V3 = 0x3,
    V4 = 0x4,
    V5 = 0x5,
    V6 = 0x6,
    V7 = 0x7,
    V8 = 0x8,
    V9 = 0x9,
    VA = 0xa,
    VB = 0xb,
    VC = 0xc,
    VD = 0xd,
    VE = 0xe,
    VF = 0xf,
}

impl fmt::Display for Reg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub fn reg(x: u8) -> Reg {
    match x {
        0x00 => Reg::V0,
        0x01 => Reg::V1,
        0x02 => Reg::V2,
        0x03 => Reg::V3,
        0x04 => Reg::V4,
        0x05 => Reg::V5,
        0x06 => Reg::V6,
        0x07 => Reg::V7,
        0x08 => Reg::V8,
        0x09 => Reg::V9,
        0x0A => Reg::VA,
        0x0B => Reg::VB,
        0x0C => Reg::VC,
        0x0D => Reg::VD,
        0x0E => Reg::VE,
        0x0F => Reg::VF,
        _ => panic!("Register {:02x} not defined!", x),
    }
}
