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
  /// Create a release changelog from previous tag until this one.
  #[structopt(long = "tag")]
  pub tag: Option<String>,
}

impl Release {
  pub fn exec(&self) {
    if let Some(tag) = &self.tag {
      let git = Git::new(&self.repository);
      let commits: Vec<ConventionalCommit> = git
        .get_all_commits_until_tag(&tag)
        .into_iter()
        .map(|c| ConventionalCommit::try_from(c))
        .filter(|c| c.is_ok())
        .map(|c| c.unwrap())
        .collect();
      let mut stdout = std::io::stdout();
      crate::format::angular::print_conventional_commit_release(
        &mut stdout,
        &tag,
        Some(git.get_commit_date(&tag)),
        &commits,
        crate::format::FormatOptions { show_all: true },
      )
      .unwrap();
  
    }
  }
}
