#[macro_use]
extern crate lazy_static;
use crate::commands::Kokai;
pub use crate::error::{Error, IntoError};
use structopt::StructOpt;

mod commands;
mod error;
mod format;
mod git;
mod parser;

fn main() {
  let opt = Kokai::from_args();

  if let Err(e) = opt.exec() {
    eprintln!("{}", e.message());
  }
}
