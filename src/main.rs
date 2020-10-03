use crate::commands::Kokai;
use structopt::StructOpt;

mod commands;
mod git;

fn main() {
  let opt = Kokai::from_args();

  opt.exec();
}
