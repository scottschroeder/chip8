//
// Rust Core Imports
//
use std::fmt;

//
// This Crate Imports
//

//
// Declare sub modules
//
mod fonts;

pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;
const GRAPHICS_SIZE: usize = (SCREEN_WIDTH * SCREEN_HEIGHT);
const MEM_SIZE: usize = (1024 * 4);
const KEY_SIZE: usize = 16;
pub const PROGRAM_START: usize = 0x200;

pub type MemAddr = u16;


#[derive(Debug)]
pub struct Keys([bool; KEY_SIZE]);

impl fmt::Display for Keys {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Keys [")?;
        for i in 0..KEY_SIZE {
            if self.0[i] {
                write!(f, "{:x}, ", i)?;
            }
        }
        write!(f, "]")?;
        Ok(())
    }
}

pub struct Interconnect {
    // TODO make this its own struct, for pretty printing
    pub keys: Keys,
    ram: Vec<u8>,
    pub graphics: [bool; GRAPHICS_SIZE],
    block_key: Option<u8>,
}


#[inline]
fn bit_index(byte: u8, index: usize) -> bool {
    (byte >> (7 - index) & 1) == 1
}

impl Interconnect {
    pub fn init() -> Self {
        let mut ic = Interconnect {
            keys: Keys([false; 16]),
            ram: vec![0; MEM_SIZE],
            graphics: [false; GRAPHICS_SIZE],
            block_key: None,
        };
        for idx in 0..fonts::CHAR_SPRITES.len() {
            ic.ram[idx + fonts::FONTS_START] = fonts::CHAR_SPRITES[idx];
        }
        ic
    }

    pub fn get_font(&self, char: u8) -> usize {
        fonts::FONTS_START + char as usize * fonts::FONT_SIZE
    }

    #[inline]
    fn map_screen(&self, idx: usize, idy: usize) -> Option<usize> {
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
                    debug!("draw_sprite: out of bounds ({}, {})", idx + col, idy + row);
                }
            }
        }
        pixel_collision
    }

    pub fn reset_keys(&mut self) {
        self.keys.0 = [false; 16];
        self.block_key = None;
    }

    pub fn set_key(&mut self, key: usize) {
        debug!("set_key key {:02x}", key);
        self.block_key = Some(key as u8);
        self.keys.0[key] = true;
    }

    pub fn check_key(&self, key: usize) -> bool {
        self.keys.0[key]
    }

    pub fn get_key(&mut self) -> Option<u8> {
        self.block_key.take()
    }

    //    pub fn load_rom(&mut self, path: PathBuf) -> Result<usize> {
//        let mut file = fs::File::open(&path)?;
//        let bytes = file.read(&mut self.ram[PROGRAM_START..])?;
//        info!("load_rom: file {} size {}", path, bytes);
//        Ok(bytes)
//    }
    pub fn program_mem(&mut self) -> &mut [u8] {
        &mut self.ram[PROGRAM_START..]
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
