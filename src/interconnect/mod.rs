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

pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;
const GRAPHICS_SIZE: usize = (SCREEN_WIDTH * SCREEN_HEIGHT);
const MEM_SIZE: usize = (1024 * 4);
const FONTS_START: usize = 0;
const FONT_SIZE: usize = 5;
const NFONTS: usize = 16;

const CHAR_SPRITES: [u8; 80] = [
    // 0
    0b11110000,
    0b10010000,
    0b10010000,
    0b10010000,
    0b11110000,

    // 1
    0b00100000,
    0b01100000,
    0b00100000,
    0b00100000,
    0b01110000,

    // 2
    0b11110000,
    0b00010000,
    0b11110000,
    0b10000000,
    0b11110000,

    // 3
    0b11110000,
    0b00010000,
    0b11110000,
    0b00010000,
    0b11110000,

    // 4
    0b10010000,
    0b10010000,
    0b11110000,
    0b00010000,
    0b00010000,

    // 5
    0b11110000,
    0b10000000,
    0b11110000,
    0b00010000,
    0b11110000,

    // 6
    0b11110000,
    0b10000000,
    0b11110000,
    0b10010000,
    0b11110000,

    // 7
    0b11110000,
    0b00010000,
    0b00100000,
    0b01000000,
    0b01000000,

    // 8
    0b11110000,
    0b10010000,
    0b11110000,
    0b10010000,
    0b11110000,

    // 9
    0b11110000,
    0b10010000,
    0b11110000,
    0b00010000,
    0b11110000,

    // A
    0b11110000,
    0b10010000,
    0b11110000,
    0b10010000,
    0b10010000,

    // B
    0b11100000,
    0b10010000,
    0b11100000,
    0b10010000,
    0b11100000,

    // C
    0b11110000,
    0b10000000,
    0b10000000,
    0b10000000,
    0b11110000,

    // D
    0b11100000,
    0b10010000,
    0b10010000,
    0b10010000,
    0b11100000,

    // E
    0b11110000,
    0b10000000,
    0b11110000,
    0b10000000,
    0b11110000,

    // F
    0b11110000,
    0b10000000,
    0b11110000,
    0b10000000,
    0b10000000,
];

pub struct Interconnect {
    keys: [bool; 16],
    ram: Vec<u8>,
    pub graphics: [bool; GRAPHICS_SIZE],
    block_key: Option<u8>,
    logger: slog::Logger,
}


#[inline]
fn bit_index(byte: u8, index: usize) -> bool {
    (byte >> (7-index) & 1) == 1
}

impl Interconnect {
    pub fn init(logger: slog::Logger) -> Self {
        let mut ic = Interconnect {
            keys: [false; 16],
            ram: vec![0; MEM_SIZE],
            graphics: [false; GRAPHICS_SIZE],
            block_key: None,
            logger: logger,
        };
        for idx in 0..CHAR_SPRITES.len() {
            ic.ram[idx + FONTS_START] = CHAR_SPRITES[idx];
        }
        ic
    }

    pub fn display_keys(&self) -> String {
        let mut result = "Keys [".to_string();
        for i in 0..16 {
            if self.keys[i] {
                result.push_str(&format!("{:x}, ", i));
            }
        }
        result.push_str(&format!("]"));
        result
    }

    pub fn get_font(&self, char: u8) -> usize {
        FONTS_START + char as usize * FONT_SIZE
    }

    #[inline]
    fn map_screen(&self, idx: usize, idy: usize) -> Option<usize> {
        // if idx > SCREEN_WIDTH || idy > SCREEN_HEIGHT {
        //     return None
        // }
        //debug!(self.logger, "map_screen"; "loc" => format!("({}, {})", idx, idy));
        Some((idy % SCREEN_HEIGHT) * SCREEN_WIDTH + (idx % SCREEN_WIDTH))
    }

    pub fn draw_sprite(&mut self, loc: usize, idx: usize, idy: usize, sprite_size: usize) -> bool {
        let mut sprite_ptr = loc;
        let mut pixel_collision = false;
        for row in 0..sprite_size {
            let sprite_byte = self.ram[sprite_ptr];
            sprite_ptr += 1;
            for col in 0..8 {
                let sprite_pixel = bit_index(sprite_byte, col);
                if let Some(coord) = self.map_screen(idx + col, idy + row) {
                    let current_pixel = self.graphics[coord];
                    let new_pixel = match (current_pixel, sprite_pixel) {
                        (true, true) => {
                            pixel_collision = true;
                            false
                        }
                        (false, true) => true,
                        (true, false) => true,
                        (false, false) => false,
                    };
                    self.graphics[coord] = new_pixel;
                } else {
                    debug!(self.logger, "draw_sprite"; "out of bounds" => format!("({}, {})", idx+col , idy+row));
                }
            }
        }
        pixel_collision
    }

    pub fn reset_keys(&mut self) {
        self.keys = [false; 16];
        self.block_key = None;
    }

    pub fn set_key(&mut self, key: usize) {
        debug!(self.logger, "set_key"; "key" => format!("{:02x}", key));
        self.block_key = Some(key as u8);
        self.keys[key] = true;
    }

    pub fn check_key(&self, key: usize) -> bool {
        self.keys[key]
    }

    pub fn get_key(&mut self) -> Option<u8> {
        self.block_key.take()
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
        self.graphics = [false; GRAPHICS_SIZE];
    }

    pub fn read_halfword(&self, addr: MemAddr) -> u16 {
        let x = self.ram[addr as usize];
        let y = self.ram[(addr + 1) as usize];
        (x as u16) << 8 | y as u16
    }
}
