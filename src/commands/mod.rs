use crate::git::Git;
use crate::parser::ConventionalCommit;
use std::convert::TryFrom;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "kokai", author, about)]
pub struct Kokai {
  /// Path to the git repository
  #[structopt(default_value = ".")]
  pub repository: String,
  /// Create a release changelog from previous tag until this one.
  #[structopt(long = "tag")]
  pub tag: Option<String>,
  /// Create a full changelog for the entire project.
  #[structopt(long = "changelog")]
  pub changelog: bool,
  /// Create a full changelog for the entire project.
  #[structopt(long = "test")]
  pub test: bool,
}

impl Kokai {
  pub fn exec(self) {
    let tag = self.tag.unwrap();
    if self.changelog {
      for c in Git::new(&self.repository).get_all_commits_before(&tag) {
        if let Ok(c) = ConventionalCommit::try_from(c) {
          println!("{}", c);
        }
      }
    } else {
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
