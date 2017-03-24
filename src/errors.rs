use std::io;


// impl From<Error> for RedisError {
//     fn from(e: Error) -> RedisError {
//         (redis::ErrorKind::TypeError, "storyestimate error", e.description().to_owned()).into()
//     }
// }

// Create the Error, ErrorKind, ResultExt, and Result types
error_chain! {
    foreign_links {
        IOError(io::Error) #[doc = "A wrapper around the `std::io::Error`"];
    }
    errors {
        UnrecognizedOpcode(b1: u8, b2: u8) {
            description("Could not disassemble Opcode")
                display("Opcode: 0x{:02x}{:02x}", b1, b2)
        }
    }
}
