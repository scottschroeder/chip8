//
// Rust Core Imports
//

//
// Third Party Imports
//
use slog;
use rand;

//
// Declare sub modules
//
mod opcodes;
mod register;

//
// Public Exports
//
pub use self::opcodes::disassemble;
pub use self::opcodes::Opcode;
pub use self::register::{Reg, reg};
use emulator::{MemAddr, PROGRAM_START};
use interconnect::Interconnect;


#[derive(Debug)]
pub struct Cpu {
    gpregs: [u8; 16],
    stack: [MemAddr; 16],
    vi: MemAddr,
    pc: MemAddr,
    sp: usize,
    delay: u8,
    sound: u8,
    logger: slog::Logger,
}

impl Cpu {
    pub fn init(logger: slog::Logger) -> Self {
        Cpu {
            gpregs: [0u8; 16],
            stack: [0u16; 16],
            vi: 0,
            pc: PROGRAM_START as _,
            sp: 0,
            delay: 0,
            sound: 0,
            logger: logger,
        }
    }

    #[inline]
    fn reg(&mut self, reg: Reg) -> &mut u8 {
        &mut self.gpregs[reg as usize]
    }

    fn execute_opcode(&mut self, opcode: &Opcode, interconnect: &mut Interconnect) {
        match opcode {
            &Opcode::ClearScreen => interconnect.clear_sceen(),
            &Opcode::Return => {
                self.sp -= 1;
                self.pc = self.stack[self.sp];
            }
            &Opcode::SysAddr(addr) => panic!("SysAddr({:04x}) Opcode not Implemented", addr),
            &Opcode::JumpAddr(addr) => self.pc = addr,
            &Opcode::CallAddr(addr) => {
                self.stack[self.sp] = self.pc;
                self.sp += 1;
                self.pc = addr
            }
            &Opcode::SkipEqByte(x, byte) => {
                if *self.reg(x) == byte {
                    self.pc += 2;
                }
            }
            &Opcode::SkipNEqByte(x, byte) => {
                if *self.reg(x) != byte {
                    self.pc += 2;
                }
            }
            &Opcode::SkipEqReg(x, y) => {
                let value = *self.reg(y);
                if *self.reg(x) == value {
                    self.pc += 2;
                }
            }
            &Opcode::LoadByte(x, byte) => {
                *self.reg(x) = byte;
            }
            &Opcode::AddByte(x, byte) => {
                let (z, overflow) = self.reg(x).overflowing_add(byte);
                *self.reg(x) = z;
                if overflow {
                    *self.reg(Reg::VF) = 1;
                } else {
                    *self.reg(Reg::VF) = 0;
                }
            }
            &Opcode::LoadReg(x, y) => {
                *self.reg(x) = *self.reg(y);
            }
            &Opcode::BitOr(x, y) => {
                *self.reg(x) = *self.reg(x) | *self.reg(y);
            }
            &Opcode::BitAnd(x, y) => {
                *self.reg(x) = *self.reg(x) & *self.reg(y);
            }
            &Opcode::BitXor(x, y) => {
                *self.reg(x) = *self.reg(x) ^ *self.reg(y);
            }
            &Opcode::MathAdd(x, y) => {
                let (z, overflow) = self.reg(x).overflowing_add(*self.reg(y));
                *self.reg(x) = z;
                if overflow {
                    *self.reg(Reg::VF) = 1;
                } else {
                    *self.reg(Reg::VF) = 0;
                }
            }
            &Opcode::MathSub(x, y) => {
                let (z, overflow) = self.reg(x).overflowing_sub(*self.reg(y));
                *self.reg(x) = z;
                if overflow {
                    *self.reg(Reg::VF) = 1;
                } else {
                    *self.reg(Reg::VF) = 0;
                }
            }
            &Opcode::ShiftRight(x, y) => {
                *self.reg(x) = *self.reg(y) >> 1;
            }
            &Opcode::MathSubN(x, y) => {
                let (z, overflow) = self.reg(y).overflowing_sub(*self.reg(x));
                *self.reg(x) = z;
                if overflow {
                    *self.reg(Reg::VF) = 1;
                } else {
                    *self.reg(Reg::VF) = 0;
                }
            }
            &Opcode::ShiftLeft(x, y) => {
                *self.reg(x) = *self.reg(y) << 1;
            }
            &Opcode::SkipNEqReg(x, y) => {
                let value = *self.reg(y);
                if *self.reg(x) != value {
                    self.pc += 2;
                }
            }
            &Opcode::MemLoad(addr) => {
                self.vi = addr;
            }
            &Opcode::JumpAddV0(addr) => self.pc = addr + *self.reg(Reg::V0) as u16,
            &Opcode::Rand(x, byte) => {
                let randombyte = rand::random::<u8>();
                *self.reg(x) = randombyte & byte;
            }
            &Opcode::Draw(x, y, byte) => {
                interconnect.draw_sprite(self.vi as _,
                                         *self.reg(x) as _,
                                         *self.reg(y) as _,
                                         byte as _);
            }
            &Opcode::KeyEqSkip(x) => {
                if interconnect.check_key(*self.reg(x) as _) {
                    self.pc += 2;
                }
            }
            &Opcode::KeyNEqSkip(x) => {
                if !interconnect.check_key(*self.reg(x) as _) {
                    self.pc += 2;
                }
            }
            &Opcode::DelayGet(x) => {
                *self.reg(x) = self.delay;
            }
            &Opcode::KeyGet(x) => {
                *self.reg(x) = interconnect.get_key();
            }
            &Opcode::DelaySet(x) => {
                self.delay = *self.reg(x) as _;
            }
            &Opcode::SoundSet(x) => {
                self.sound = *self.reg(x) as _;
            }
            &Opcode::MemAdd(x) => {
                self.vi += *self.reg(x) as u16;
            }
            &Opcode::MemSprite(x) => {
                self.vi = interconnect.get_font(*self.reg(x)) as _;
            }
            &Opcode::BCD(x) => {
                // 231 = M[vi] = 2; M[vi+1] = 3; M[vi+2] = 1
                let n = *self.reg(x);
                let ones = n % 10;
                let tens = (n / 10) % 10;
                let hundreds = *self.reg(x) / 100;

                interconnect.write_byte(self.vi, hundreds);
                interconnect.write_byte(self.vi + 1, tens);
                interconnect.write_byte(self.vi + 2, ones);
            }
            &Opcode::RegDump(x) => {
                for idx in 0..(x as usize) {
                    interconnect.write_byte(self.vi + (idx as u16), *self.reg(reg(idx as _)));
                }
            }
            &Opcode::RegLoad(x) => {
                for idx in 0..(x as usize) {
                    *self.reg(reg(idx as _)) = interconnect.read_byte(self.vi + (idx as u16));
                }
            }
        }
    }

    pub fn run_cycle(&mut self, interconnect: &mut Interconnect) {
        let instr = interconnect.read_halfword(self.pc);
        self.pc += 2; // We moved two bytes
        let opcode = disassemble(instr).unwrap();
        debug!(self.logger, "run_cycle"; "opcode" => format!("{}", opcode));
        self.execute_opcode(&opcode, interconnect);
    }
}
