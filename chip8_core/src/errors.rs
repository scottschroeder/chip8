use std::error::Error;
use std::fmt;


#[derive(Debug)]
pub enum Chip8Error {
    UnrecognizedOpcode(u16)
}


impl fmt::Display for Chip8Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for Chip8Error {
    fn description(&self) -> &str {
        "Unable to parse halfword as OpCode"
    }

    fn cause(&self) -> Option<&Error> {
        None
    }

}