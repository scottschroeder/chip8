#[macro_use]
extern crate error_chain;
extern crate slog;
extern crate slog_term;

mod loader;
mod errors;

use errors::*;

pub use loader::load_rom;


#[test]
fn it_works() {}
