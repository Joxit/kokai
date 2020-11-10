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
  /// Create a release changelog from previous tag until this one. Can be a tag, commit hash or branch.
  #[structopt(long = "tag")]
  pub tag: String,
  /// Explicit name for the release. Useful when tag is a commit or HEAD.
  #[structopt(long = "name")]
  pub name: Option<String>,
}

impl Release {
  pub fn exec(&self) {
    let name = if let Some(name) = &self.name {
      name
    } else {
      &self.tag
    };
    let git = Git::new(&self.repository);
    let commits: Vec<ConventionalCommit> = git
      .get_all_commits_until_tag(&self.tag)
      .into_iter()
      .map(|c| ConventionalCommit::try_from(c))
      .filter(|c| c.is_ok())
      .map(|c| c.unwrap())
      .collect();
    let mut stdout = std::io::stdout();
    crate::format::angular::print_conventional_commit_release(
      &mut stdout,
      &name,
      Some(git.get_commit_date(&self.tag)),
      &commits,
      crate::format::FormatOptions { show_all: true },
    )
    .unwrap();
  }
}
