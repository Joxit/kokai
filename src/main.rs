use crate::commands::Kokai;
use structopt::StructOpt;

mod commands;

#[derive(Debug, StructOpt)]
#[structopt(name = "kokai", author, about)]
pub struct ApplicationArguments {
  #[structopt(subcommand)]
  pub command: Kokai,
}

fn main() {
  let opt = ApplicationArguments::from_args();

  opt.command.exec();
}
