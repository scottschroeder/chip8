#![warn(missing_docs)]
#![recursion_limit = "1024"]

//! Everything needed to build a Chip8 System

#[macro_use]
extern crate error_chain;
#[macro_use]
pub extern crate slog;
extern crate slog_stdlog;
extern crate rand;

mod cpu;
mod emulator;
mod errors;
mod interconnect;


pub use emulator::Chip8;
pub use errors::*;


#[test]
fn it_works() {}
