pub use crate::parser::angular::AngularCommit;
mod angular;

#[derive(Debug, Clone)]
pub enum CommitType {
  BugFix,
  Feature,
  Perf,
  Revert,
}

impl std::convert::TryFrom<&str> for CommitType {
  type Error = ();
  fn try_from(commit_type: &str) -> Result<CommitType, ()> {
    match commit_type {
      "fix" => Ok(CommitType::BugFix),
      "feat" => Ok(CommitType::Feature),
      "perf" => Ok(CommitType::Perf),
      "revert" => Ok(CommitType::Revert),
      _ => Err(()),
    }
  }
}
