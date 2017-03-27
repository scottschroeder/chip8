//
// Rust Core Imports
//
use std::path::PathBuf;
use std::fs;
use std::io::Read;
use std::thread;
use std::time::{Duration, Instant};

//TODO REMOVE
use std::io;

//
// Third Party Imports
//
use slog;
use slog_stdlog;
use slog::DrainExt;
use minifb::{WindowOptions, Window, Key, KeyRepeat, Scale};

//
// This Crate Imports
//
use errors::*;
use cpu;
use interconnect::{Interconnect, SCREEN_WIDTH, SCREEN_HEIGHT};

pub const PROGRAM_START: usize = 0x200;
pub const NS_IN_SECOND: u64 = 1000000000;
pub const CPU_CYCLE_NS: u64 = 2000000; //500Hz
pub const TIMER_CYCLE_NS: u64 = 16666667;
pub type MemAddr = u16;

/// The interface to the core Chip8 system.
pub struct Chip8 {
    logger: slog::Logger,
    cpu: cpu::Cpu,
    interconnect: Interconnect,
    window: Window,
    start_time: Instant,
    cpu_cycles: u64,
    timer_ticks: u64,
    debug_mode: bool,
    breakpoints: Vec<u16>,
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
    /// `logger = None`, will use the standard `log` crate.
    pub fn init(logger: Option<slog::Logger>) -> Self {
        let emu_logger = logger.unwrap_or(slog::Logger::root(slog_stdlog::StdLog.fuse(), o!()));
        let cpu_logger = emu_logger.new(o!("device" => "cpu"));
        let int_logger = emu_logger.new(o!("device" => "interconnect"));
        Chip8 {
            logger: emu_logger,
            cpu: cpu::Cpu::init(cpu_logger),
            interconnect: Interconnect::init(int_logger),
            cpu_cycles: 0,
            timer_ticks: 0,
            debug_mode: false,
            start_time: Instant::now(),
            breakpoints: Vec::new(),
            window: Window::new("Chip8",
                                SCREEN_WIDTH,
                                SCREEN_HEIGHT,
                                WindowOptions {
                                    borderless: false,
                                    title: true,
                                    resize: false,
                                    scale: Scale::X32,
                                })
                .unwrap(),
        }
    }

    /// Load a Chip8 ROM from the filesystem
    pub fn load_rom(&mut self, path: PathBuf) -> Result<usize> {
        self.interconnect.load_rom(path)
    }


    /// Run the emulator
    pub fn run(&mut self) {
        self.start_time = Instant::now();
        let naptime = Duration::from_millis(3);
        let mut buffer = [0u32; SCREEN_HEIGHT * SCREEN_WIDTH];

        while self.window.is_open() && !self.window.is_key_down(Key::Escape) {
            let emulation_time = self.start_time.elapsed();
            let emulation_ns = emulation_time.as_secs() * NS_IN_SECOND +
                               emulation_time.subsec_nanos() as u64;
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
                    debug!(self.logger, "debug_cpu"; "keys" => self.interconnect.display_keys());
                    println!("{}", self.cpu);
                    for i in -5..10 {
                        let memaddr = (self.cpu.pc as isize + 2*i) as u16;
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
                        }
                        Err(e) => {
                            println!("{:?}", e);
                        }

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

    fn draw_screen(&mut self, buffer: &mut [u32; SCREEN_WIDTH * SCREEN_HEIGHT]) {
        for (idx, pixel) in self.interconnect.graphics.iter().enumerate() {
            buffer[idx] = if *pixel { 0x00ffffff } else { 0 };
        }
        self.window.update_with_buffer(buffer);
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
