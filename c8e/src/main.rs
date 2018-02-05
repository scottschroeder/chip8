//
// Crate Imports
//

extern crate chip8_core;
extern crate clap;
#[macro_use]
extern crate log;
extern crate minifb;
extern crate pretty_env_logger;

mod emulator;

//
// Rust Core Imports
//
use std::path::PathBuf;

//
// Third Party Imports
//
use clap::{App, Arg};

fn main() {
    pretty_env_logger::init();

    let matches = App::new("Chip 8 Emulator")
        .version("0.1.0")
        .author("Scott Schroeder <scottschroeder@sent.com>")
        .about("c8e is pronounced 'Sadie'")
        .arg(
            Arg::with_name("rom_path")
                .short("r")
                .long("rom")
                .value_name("FILE")
                .help("File path for ROM to load")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("debugger")
                .short("d")
                .help("Run this ROM with the console debugger"),
        )
        .arg(
            Arg::with_name("disassemble")
                .short("p")
                .help("Dump the instructions for this ROM"),
        )
        .get_matches();

    let rom_path = matches.value_of("rom_path").unwrap(); //Required arg
    let mut chip8 = emulator::Chip8::init();
    let rom_bytes = chip8.load_rom(PathBuf::from(rom_path)).unwrap();

    if matches.is_present("debugger") {
        chip8.set_debug(true)
    } else {
        chip8.set_debug(false)
    }
    if matches.is_present("disassemble") {
        chip8.disassemble(rom_bytes);
    } else {
        chip8.run();
    }
    //println!("{:?}", chip8);
    //mem_dump(&chip8.rom[..], 0);
}
