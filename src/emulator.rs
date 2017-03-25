//
// Rust Core Imports
//
use std::path::PathBuf;
use std::fs;
use std::io::Read;

//
// Third Party Imports
//
use slog;
use slog_stdlog;
use slog::DrainExt;

//
// This Crate Imports
//
use errors::*;
use cpu;
use interconnect::Interconnect;

pub const PROGRAM_START: usize = 0x200;
pub type MemAddr = u16;

/// The interface to the core Chip8 system.
pub struct Chip8 {
    logger: slog::Logger,
    cpu: cpu::Cpu,
    interconnect: Interconnect,
}

impl Chip8 {
    /// Initialize the `Chip8` system
    ///
    /// `logger = None`, will use the standard `log` crate.
    pub fn init(logger: Option<slog::Logger>) -> Self {
        let emu_logger = logger.unwrap_or(slog::Logger::root(slog_stdlog::StdLog.fuse(), o!()));
        let cpu_logger = emu_logger.new(o!("device" => "cpu"));
        let int_logger = emu_logger.new(o!("device" => "interconnect"));
        Chip8 {
            logger: emu_logger,
            cpu: cpu::Cpu::init(cpu_logger),
            interconnect: Interconnect::init(int_logger),
        }
    }

    /// Load a Chip8 ROM from the filesystem
    pub fn load_rom(&mut self, path: PathBuf) -> Result<usize> {
        self.interconnect.load_rom(path)
    }

    /// Run the emulator
    pub fn run(&mut self) {
        loop {
            self.cpu.run_cycle(&mut self.interconnect);
        }
    }

    pub fn disassemble(&self, total: usize) {
        let mut idx = PROGRAM_START;
        while idx + 1 < total + PROGRAM_START {
            let instr = self.interconnect.read_halfword(idx as _);
            print!("0x:{:04x} (0x{:04x}):\t", idx, instr);
            idx += 2;
            match cpu::disassemble(instr) {
                Ok(opcode) => println!("{}", opcode),
                Err(e) => println!("UNKNOWN {}", e),
            }
        }

    }
}
