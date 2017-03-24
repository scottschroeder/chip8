//
// Crate Imports
//
extern crate c8lib;
extern crate clap;
#[macro_use]
extern crate slog;
extern crate slog_term;


//
// Rust Core Imports
//
use std::path::PathBuf;

//
// Third Party Imports
//
use clap::{Arg, App};
use slog::DrainExt;

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
    let log = slog::Logger::root(slog_term::streamer().full().build().fuse(),
                                 o!("c8e_version" => env!("CARGO_PKG_VERSION")));

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
    let mut chip8 = c8lib::Chip8::init(Some(log));
    chip8.load_rom(PathBuf::from(rom_path)).unwrap();
    mem_dump(&chip8.rom[..], 0);
    chip8.disassemble(0, 1000);
}
