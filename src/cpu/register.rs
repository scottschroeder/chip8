//
// Rust Core Imports
//

//
// Third Party Imports
//

//
// This Crate Imports
//
//use errors::*;

pub const GPREGS: [Reg; 16] = [
    Reg::V0,
    Reg::V1,
    Reg::V2,
    Reg::V3,
    Reg::V4,
    Reg::V5,
    Reg::V6,
    Reg::V7,
    Reg::V8,
    Reg::V9,
    Reg::VA,
    Reg::VB,
    Reg::VC,
    Reg::VD,
    Reg::VE,
    Reg::VF,
];

#[derive(Debug, Clone, Copy)]
pub enum Reg{
    V0,
    V1,
    V2,
    V3,
    V4,
    V5,
    V6,
    V7,
    V8,
    V9,
    VA,
    VB,
    VC,
    VD,
    VE,
    VF,
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
        _ => panic!("Register {:02x} not defined!", x)
    }
}
