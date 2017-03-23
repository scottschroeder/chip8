use std::path::PathBuf;
use std::fs;
use std::io::Read;

use errors::*;

pub fn load_rom(path: PathBuf) -> Result<Vec<u8>> {
    let metadata = fs::metadata(&path)?;
    let mut file = fs::File::open(&path)?;
    let mut rom: Vec<u8> = Vec::with_capacity(metadata.len() as _);
    file.read_to_end(&mut rom)?;
    Ok(rom)
}
