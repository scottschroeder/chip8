//
// Crate Imports
//
extern crate c8lib;


//
// Rust Core Imports
//
use std::path::PathBuf;
extern crate clap;
use clap::{Arg, App};

const HEXDUMP_COLS: usize = 16;

fn mem_dump(mem: &[u8], start_offset: usize) {
    let max_bytes = HEXDUMP_COLS * 16; //rows
    let mut spacer;
    for (idx, byte) in mem.iter().enumerate().take(max_bytes) {
        if idx % HEXDUMP_COLS == 0 {
            let addr = idx + start_offset;
            print!("\n0x{:08x}:\t", addr);
        }
        if idx % 2 == 0 {
            spacer = ""
        } else {
            spacer = " "
        }
        print!("{:02x}{}", byte, spacer);
    }
    println!();
}

fn main() {
    let matches = App::new("Chip 8 Emulator")
        .version("0.1.0")
        .author("Scott Schroeder <scottschroeder@sent.com>")
        .about("c8e is pronounced 'Sadie'")
        .arg(Arg::with_name("rom_path")
            .short("r")
            .long("rom")
            .value_name("FILE")
            .help("File path for ROM to load")
            .takes_value(true)
            .required(true))
        .get_matches();

    let rom_path = matches.value_of("rom_path").unwrap(); //Required arg
    let rom = c8lib::load_rom(PathBuf::from(rom_path)).unwrap();
    mem_dump(&rom[..], 0);
}
