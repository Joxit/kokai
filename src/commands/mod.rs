use crate::commands::changelog::Changelog;
use crate::commands::completion::Completion;
use crate::commands::release::Release;
use crate::Error;
use structopt::StructOpt;

mod changelog;
mod completion;
mod release;

#[derive(Debug, StructOpt)]
pub enum Kokai {
  /// Create a release changelog for a specified tag.
  #[structopt(name = "release")]
  Release(Release),
  /// Create a full changelog of your project.
  #[structopt(name = "changelog")]
  Changelog(Changelog),
  /// Generate autocompletion file for your shell.
  #[structopt(name = "completion")]
  Completion(Completion),
}

impl Kokai {
  pub fn exec(&self) -> Result<(), Error> {
    match self {
      Kokai::Release(executable) => executable.exec()?,
      Kokai::Changelog(executable) => executable.exec()?,
      Kokai::Completion(executable) => executable.exec()?,
    }
    Ok(())
  }
}
