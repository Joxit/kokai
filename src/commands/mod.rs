use crate::git::Git;
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
}

impl Kokai {
  pub fn exec(self) {
    let tag = self.tag.unwrap();
    if self.changelog {
      for c in Git::new(&self.repository).get_all_commits_before(&tag) {
        println!("{:?}", c);
      }
    }
  }
}
