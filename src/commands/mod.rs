use crate::commands::changelog::Changelog;
use crate::commands::release::Release;
use structopt::StructOpt;

mod release;
mod changelog;

#[derive(Debug, StructOpt)]
pub enum Kokai {
  /// Create a release changelog for a specified tag.
  #[structopt(name = "release")]
  Release(Release),
  /// Create a full changelog for the full history.
  #[structopt(name = "changelog")]
  Changelog(Changelog),
}

impl Kokai {
  pub fn exec(&self) {
    match self {
      Kokai::Release(executable) => executable.exec(),
      Kokai::Changelog(executable) => executable.exec(),
    }
  }
}