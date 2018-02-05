#![warn(missing_docs)]

//! Everything needed to build a Chip8 System

extern crate rand;

#[macro_use]
extern crate log;
//#[macro_use]
//extern crate failure;

mod errors;
pub mod cpu;
pub mod interconnect;

