use crate::error::{Error, IntoError};
use crate::git::Git;
use crate::parser::ConventionalCommit;
use std::convert::TryFrom;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "release", author, about)]
pub struct Release {
  /// Path to the git repository
  #[structopt(default_value = ".")]
  pub repository: String,
  /// Create a release changelog from previous tag until this ref. Can be a tag, commit hash or branch.
  #[structopt(long = "ref")]
  pub r#ref: String,
  /// Explicit name for the release. Useful when tag is a commit or HEAD.
  #[structopt(long = "name")]
  pub name: Option<String>,
  /// Get the tag of the ref commit and use it as a release name. This is like `git describe --tags --exact-match`
  #[structopt(long = "tag-from-ref")]
  pub tag_from_ref: bool,
}

impl Release {
  pub fn exec(&self) -> Result<(), Error> {
    let git = Git::new(&self.repository);
    let name = if let Some(name) = &self.name {
      name.clone()
    } else if self.tag_from_ref {
      git.get_tag_of(&self.r#ref)?
    } else {
      self.r#ref.clone()
    };
    let commits: Vec<ConventionalCommit> = git
      .get_all_commits_until_tag(&self.r#ref)?
      .into_iter()
      .map(|c| ConventionalCommit::try_from(c))
      .filter(|c| c.is_ok())
      .map(|c| c.unwrap())
      .collect();
    let mut stdout = std::io::stdout();
    crate::format::angular::print_conventional_commit_release(
      &mut stdout,
      &name,
      Some(git.get_commit_date(&self.r#ref)?),
      &commits,
      crate::format::FormatOptions { show_all: true },
    )
    .into_error()
  }
}
