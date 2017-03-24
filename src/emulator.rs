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

/// The interface to the core Chip8 system.
pub struct Chip8 {
    logger: slog::Logger,

    /// The raw bytes of the ROM
    /// This must be loaded by the user of the Chip8
    /// by calling `load_rom`
    pub rom: Vec<u8>,
}

impl Chip8 {
    /// Initialize the `Chip8` system
    ///
    /// `logger = None`, will use the standard `log` crate.
    pub fn init(logger: Option<slog::Logger>) -> Self {
        Chip8 {
            logger: logger.unwrap_or(slog::Logger::root(slog_stdlog::StdLog.fuse(), o!())),
            rom: Vec::new(),
        }
    }

    /// Load a Chip8 ROM from the filesystem
    pub fn load_rom(&mut self, path: PathBuf) -> Result<()> {
        let mut file = fs::File::open(&path)?;
        let file_size = file.metadata()?.len() as usize;
        self.rom.reserve_exact(file_size);
        file.read_to_end(&mut self.rom)?;
        info!(self.logger, "load_rom"; "file" => path.as_path().to_str(), "size" => file_size);
        Ok(())
    }

    pub fn disassemble(&self, start: usize, total: usize) {
        let mut idx = 0;
        while start + idx + 1 < self.rom.len() && idx/2 < total {
            let x = self.rom[idx];
            let y = self.rom[idx + 1];
            idx += 2;
            print!("0x:{:04x} (0x{:02x}{:02x}):\t", start+idx, x, y);
            match cpu::disassemble(x, y) {
                Ok(opcode) => println!("{:?}", opcode),
                Err(e) => println!("UNKNOWN {}", e),
            }
        }

    }
}
