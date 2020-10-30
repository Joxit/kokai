pub use crate::parser::angular::AngularCommit;
mod angular;

#[derive(Debug, Clone)]
pub enum CommitType {
  Fix,
  Feature,
  Perf,
  Revert,
  Build,
  Chore,
  Ci,
  Docs,
  Style,
  Refactor,
  Test,
}

impl std::convert::TryFrom<&str> for CommitType {
  type Error = String;
  fn try_from(commit_type: &str) -> Result<CommitType, String> {
    match commit_type {
      "fix" => Ok(CommitType::Fix),
      "feat" => Ok(CommitType::Feature),
      "perf" => Ok(CommitType::Perf),
      "revert" => Ok(CommitType::Revert),
      "build" => Ok(CommitType::Build),
      "chore" => Ok(CommitType::Chore),
      "ci" => Ok(CommitType::Ci),
      "docs" => Ok(CommitType::Docs),
      "style" => Ok(CommitType::Style),
      "refactor" => Ok(CommitType::Refactor),
      "test" => Ok(CommitType::Test),
      _ => Err(format!("Unknonw type")),
    }
  }
}
