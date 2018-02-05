//
// Rust Core Imports
//
use std::fs;
use std::io::Read;
use std::path::PathBuf;
use std::thread;
use std::time::{Duration, Instant};

use std::io;

//
// Third Party Imports
//
use minifb::{Key, Scale, Window, WindowOptions};

//
// This Crate Imports
//
use chip8_core::cpu;
use chip8_core::interconnect::{Interconnect, SCREEN_HEIGHT, SCREEN_WIDTH};

const SCREEN_SCALE: usize = 16; // Must be power of 2;
const DISPLAY_WIDTH: usize = SCREEN_WIDTH * SCREEN_SCALE;
const DISPLAY_HEIGHT: usize = SCREEN_HEIGHT * SCREEN_SCALE;
const DISPLAY_SIZE: usize = DISPLAY_HEIGHT * DISPLAY_WIDTH;
const NS_IN_SECOND: u64 = 1000000000;
const CPU_CYCLE_NS: u64 = 2000000; //500Hz
const TIMER_CYCLE_NS: u64 = 16666667;
const HEXDUMP_COLS: usize = 16;

/// The interface to the core Chip8 system.
pub struct Chip8 {
    cpu: cpu::Cpu,
    interconnect: Interconnect,
    window: Window,
    start_time: Instant,
    cpu_cycles: u64,
    timer_ticks: u64,
    debug_mode: bool,
}

fn key_map(key: Key) -> Option<usize> {
    match key {
        Key::Key1 => Some(0x0),
        Key::Key2 => Some(0x1),
        Key::Key3 => Some(0x2),
        Key::Key4 => Some(0x3),
        Key::Q => Some(0x4),
        Key::W => Some(0x5),
        Key::E => Some(0x6),
        Key::R => Some(0x7),
        Key::A => Some(0x8),
        Key::S => Some(0x9),
        Key::D => Some(0xA),
        Key::F => Some(0xB),
        Key::Z => Some(0xC),
        Key::X => Some(0xD),
        Key::C => Some(0xE),
        Key::V => Some(0xF),
        _ => None,
    }
}

impl Chip8 {
    /// Initialize the `Chip8` system
    ///
    pub fn init() -> Self {
        Chip8 {
            cpu: cpu::Cpu::init(),
            interconnect: Interconnect::init(),
            cpu_cycles: 0,
            timer_ticks: 0,
            debug_mode: false,
            start_time: Instant::now(),
            window: Window::new(
                "Chip8",
                DISPLAY_WIDTH,
                DISPLAY_HEIGHT,
                WindowOptions {
                    borderless: false,
                    title: true,
                    resize: false,
                    scale: Scale::X1,
                },
            ).unwrap(),
        }
    }

    /// Load a Chip8 ROM from the filesystem
    pub fn load_rom(&mut self, path: PathBuf) -> Result<usize, io::Error> {
        let mut program_mem = self.interconnect.program_mem();
        let mut file = fs::File::open(&path)?;
        let bytes = file.read(&mut program_mem)?;
        info!("load_rom: file {} size {}", path.display(), bytes);
        Ok(bytes)
    }

    /// Run the emulator
    pub fn run(&mut self) {
        self.start_time = Instant::now();
        let naptime = Duration::from_millis(3);
        let mut buffer = [0u32; DISPLAY_SIZE]; // TODO box it

        while self.window.is_open() && !self.window.is_key_down(Key::Escape) {
            let emulation_time = self.start_time.elapsed();
            let emulation_ns =
                emulation_time.as_secs() * NS_IN_SECOND + emulation_time.subsec_nanos() as u64;
            let ideal_cpu_cycles = emulation_ns / CPU_CYCLE_NS;
            let ideal_timer_ticks = emulation_ns / TIMER_CYCLE_NS;

            self.cpu.timer(ideal_timer_ticks - self.timer_ticks);
            self.timer_ticks = ideal_timer_ticks;

            if !self.debug_mode {
                self.update_keys();
            }
            while self.cpu_cycles < ideal_cpu_cycles {
                // if self.cpu.pc == 0x0278 {
                //     self.debug_mode = true;
                // }
                if self.debug_mode {
                    debug!("{}", self.interconnect.keys);
                    println!("{}", self.cpu);
                    for i in -5..10 {
                        let memaddr = (self.cpu.pc as isize + 2 * i) as u16;
                        let instr = self.interconnect.read_halfword(memaddr);
                        if i == 0 {
                            print!("-->");
                        }
                        match cpu::disassemble(instr) {
                            Ok(opcode) => println!("\t0x{:04x} {}", memaddr, opcode),
                            Err(e) => println!("\t0x{:04x} UNRECOGNIZED {}", memaddr, e),
                        }
                    }

                    // hack to input a key in debug mode
                    // insert 0-16 to press key
                    // insert something larger than 16 to clear keys
                    let mut input_text = String::new();
                    io::stdin()
                        .read_line(&mut input_text)
                        .expect("failed to read from stdin");
                    match input_text.trim().parse::<usize>() {
                        Ok(x) => {
                            if x > 16 {
                                self.interconnect.reset_keys();
                            } else {
                                self.interconnect.set_key(x);
                            }
                        },
                        Err(e) => {
                            println!("{:?}", e);
                        },
                    }
                }
                self.cpu.run_cycle(&mut self.interconnect);
                self.cpu_cycles += 1;
                if self.debug_mode {
                    break;
                }
            }

            self.draw_screen(&mut buffer);
            thread::sleep(naptime);
        }
    }

    pub fn mem_dump(mem: &[u8], start_offset: usize) {
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
    pub fn set_debug(&mut self, status: bool) {
        self.debug_mode = status;
    }

    fn update_keys(&mut self) {
        self.interconnect.reset_keys();
        if let Some(keys) = self.window.get_keys() {
            for key in keys {
                if let Some(chip8_key) = key_map(key) {
                    self.interconnect.set_key(chip8_key)
                }
            }
        }
    }

    fn draw_screen(&mut self, buffer: &mut [u32; DISPLAY_SIZE]) {
        for dy in 0..DISPLAY_HEIGHT {
            let sy = dy / SCREEN_SCALE;
            for dx in 0..DISPLAY_WIDTH {
                let sx = dx / SCREEN_SCALE;
                let display_index = dy * DISPLAY_WIDTH + dx;
                let screen_index = sy * SCREEN_WIDTH + sx;
                buffer[display_index] = if self.interconnect.graphics[screen_index] {
                    0x00ffffff
                } else {
                    0
                };
            }
        }
        self.window.update_with_buffer(buffer);
    }

    pub fn disassemble(&self, total: usize) {
        // TODO kill
        let PROGRAM_START = 0x200;
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
