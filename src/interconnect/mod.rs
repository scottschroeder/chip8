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

//
// This Crate Imports
//
use errors::*;
use emulator::{MemAddr, PROGRAM_START};

//
// Declare sub modules
//

//
// Public Exports
//

const SCREEN_WIDTH: usize = 64;
const SCREEN_HEIGHT: usize = 32;
const GRAPHICS_SIZE: usize = (SCREEN_WIDTH * SCREEN_HEIGHT);
const MEM_SIZE: usize = (1024 * 4);
const FONTS_START: usize = 0;
const FONT_SIZE: usize = 5;
const NFONTS: usize = 16;

const CHAR_SPRITES: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

pub struct Interconnect {
    keys: [bool; 16],
    ram: Vec<u8>,
    pub graphics: [u8; GRAPHICS_SIZE],
    logger: slog::Logger,
}

impl Interconnect {
    pub fn init(logger: slog::Logger) -> Self {
        let mut ic = Interconnect {
            keys: [false; 16],
            ram: vec![0; MEM_SIZE],
            graphics: [0; GRAPHICS_SIZE],
            logger: logger,
        };
        for idx in 0..CHAR_SPRITES.len() {
            ic.ram[idx + FONTS_START] = CHAR_SPRITES[idx];
        }
        ic
    }

    pub fn get_font(&self, char: u8) -> usize {
        FONTS_START + char as usize * FONT_SIZE
    }

    pub fn draw_sprite(&mut self, loc: usize, idx: usize, idy: usize, sprite_size: usize) -> bool {
        // TODO
        false
    }

    pub fn check_key(&self, key: usize) -> bool {
        self.keys[key]
    }

    pub fn get_key(&mut self) -> u8 {
        // TODO
        panic!("HOW I MINE KEY?");
    }

    pub fn load_rom(&mut self, path: PathBuf) -> Result<usize> {
        let mut file = fs::File::open(&path)?;
        let bytes = file.read(&mut self.ram[PROGRAM_START..])?;
        info!(self.logger, "load_rom"; "file" => path.as_path().to_str(), "size" => bytes);
        Ok(bytes)
    }

    pub fn write_byte(&mut self, addr: MemAddr, byte: u8) {
        self.ram[addr as usize] = byte;
    }

    pub fn read_byte(&self, addr: MemAddr) -> u8 {
        self.ram[addr as usize]
    }

    pub fn clear_sceen(&mut self) {
        self.graphics = [0; GRAPHICS_SIZE];
    }

    pub fn read_halfword(&self, addr: MemAddr) -> u16 {
        let x = self.ram[addr as usize];
        let y = self.ram[(addr + 1) as usize];
        (x as u16) << 8 | y as u16
    }
}
