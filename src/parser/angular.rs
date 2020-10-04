use crate::git::Commit;
use crate::parser::CommitType;
use regex::Regex;

lazy_static! {
  static ref SUMMARY_REGEX: Regex =
    Regex::new("^(?P<type>fix|feat|perf|revert)(?:\\(?P<scope>.*\\))?: (?P<summary>.*)$").unwrap();
}

#[derive(Debug, Clone)]
pub struct AngularCommit {
  id: String,
  scope: Option<String>,
  summary: String,
  commit_type: CommitType,
}

impl std::convert::TryFrom<Commit> for AngularCommit {
  type Error = ();
  fn try_from(commit: Commit) -> Result<AngularCommit, ()> {
    let id = commit.id();
    if let Some(matches) = SUMMARY_REGEX.captures(commit.summary()) {
      let commit_type = matches.name("type");
      let summary = matches.name("summary");
      if commit_type.is_none() || summary.is_none() {
        return Err(());
      }
      Ok(AngularCommit {
        id: id.to_string(),
        scope: matches.name("scope").map(|s| s.as_str().to_string()),
        summary: summary.unwrap().as_str().to_string(),
        commit_type: CommitType::try_from(commit_type.unwrap().as_str())?,
      })
    } else {
      Err(())
    }
  }
}
