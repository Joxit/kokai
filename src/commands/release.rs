use crate::error::{Error, IntoError};
use crate::format::FormatURL;
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
  /// Add links to commits/issues/pr with specified url format (github/gitlab...).
  /// For commits only using github url format, use github:commits. For gitlab with commits and issues use gitlab:commits,issues.
  #[structopt(long = "add-links")]
  pub add_links: Option<String>,
  /// The git url of the project. Should be a url using http protocol for links.
  #[structopt(long = "git-url")]
  pub git_url: Option<String>,
  /// Remove emojis from headers.
  #[structopt(long = "no-emoji")]
  pub no_emoji: bool,
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
    let git_url = if let Some(git_url) = &self.git_url {
      git_url.to_string()
    } else {
      git.url()?
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
      crate::format::FormatOptions {
        show_all: true,
        emoji: !self.no_emoji,
        format_url: FormatURL::new(git_url, self.add_links.clone()),
      },
    )
    .into_error()
  }
}
