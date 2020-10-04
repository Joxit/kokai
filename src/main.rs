#[macro_use]
extern crate lazy_static;
use crate::commands::Kokai;
use structopt::StructOpt;

mod commands;
mod git;
mod parser;

fn main() {
  let opt = Kokai::from_args();

  opt.exec();
}
