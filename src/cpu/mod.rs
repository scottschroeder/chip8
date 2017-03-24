
type MemAddr = u16;

mod opcodes;
mod register;

pub use self::opcodes::disassemble;
pub use self::opcodes::Opcode;
pub use self::register::Reg;
