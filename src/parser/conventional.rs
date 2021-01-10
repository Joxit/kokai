use crate::format::formatters::Markdown;
use crate::format::FormatOptions;
use crate::git::Commit;
use regex::Regex;

lazy_static! {
  static ref SUMMARY_REGEX: Regex =
    Regex::new("^(?P<type>fix|feat|perf|revert|build|chore|ci|docs|style|refactor|test)(?:\\((?P<scope>.*)\\))?: (?P<summary>.*)$").unwrap();
}

#[derive(Debug, Clone)]
pub struct ConventionalCommit {
  pub id: String,
  pub scope: Option<String>,
  pub summary: String,
  pub commit_type: ConventionalCommitType,
}

impl std::convert::TryFrom<Commit> for ConventionalCommit {
  type Error = String;
  fn try_from(commit: Commit) -> Result<ConventionalCommit, String> {
    let id = commit.id();
    if let Some(matches) = SUMMARY_REGEX.captures(commit.summary()) {
      let commit_type = matches.name("type");
      let summary = matches.name("summary");
      if commit_type.is_none() || summary.is_none() {
        return Err(format!("Commit type is empty"));
      }
      Ok(ConventionalCommit {
        id: id.to_string(),
        scope: matches.name("scope").map(|s| s.as_str().to_string()),
        summary: summary.unwrap().as_str().to_string(),
        commit_type: ConventionalCommitType::try_from(commit_type.unwrap().as_str())?,
      })
    } else {
      Err(format!(
        "This commit message don't use the conventionnal commit"
      ))
    }
  }
}

impl std::fmt::Display for ConventionalCommit {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.commit_type)?;
    if let Some(scope) = self.scope.clone() {
      write!(f, "({})", scope)?;
    }
    let small_id = self.id.chars().take(8).collect::<String>();
    write!(f, ": {} ({})", self.summary, small_id)
  }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ConventionalCommitType {
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

impl std::convert::TryFrom<&str> for ConventionalCommitType {
  type Error = String;
  fn try_from(commit_type: &str) -> Result<ConventionalCommitType, String> {
    match commit_type {
      "fix" => Ok(ConventionalCommitType::Fix),
      "feat" => Ok(ConventionalCommitType::Feature),
      "perf" => Ok(ConventionalCommitType::Perf),
      "revert" => Ok(ConventionalCommitType::Revert),
      "build" => Ok(ConventionalCommitType::Build),
      "chore" => Ok(ConventionalCommitType::Chore),
      "ci" => Ok(ConventionalCommitType::Ci),
      "docs" => Ok(ConventionalCommitType::Docs),
      "style" => Ok(ConventionalCommitType::Style),
      "refactor" => Ok(ConventionalCommitType::Refactor),
      "test" => Ok(ConventionalCommitType::Test),
      _ => Err(format!("Unknonw type")),
    }
  }
}

impl std::fmt::Display for ConventionalCommitType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let t = match self {
      ConventionalCommitType::Fix => "fix",
      ConventionalCommitType::Feature => "feat",
      ConventionalCommitType::Perf => "perf",
      ConventionalCommitType::Revert => "revert",
      ConventionalCommitType::Build => "build",
      ConventionalCommitType::Chore => "chore",
      ConventionalCommitType::Ci => "ci",
      ConventionalCommitType::Docs => "docs",
      ConventionalCommitType::Style => "style",
      ConventionalCommitType::Refactor => "refactor",
      ConventionalCommitType::Test => "test",
    };
    write!(f, "{}", t)
  }
}

impl Markdown for ConventionalCommit {
  fn markdown(&self, opts: &FormatOptions) -> String {
    let scope = if let Some(scope) = self.scope.clone() {
      format!("{}: ", scope.bold())
    } else {
      String::new()
    };
    let small_id = self.id.chars().take(8).collect::<String>().code();
    let id = if let Some(commit_url) = &opts.commit_url(&self.id) {
      small_id.link(commit_url)
    } else {
      small_id
    };
    let mut summary = self.summary.to_string();
    if let Some(issues) = opts.get_all_issues(&summary) {
      for issue in issues {
        summary = summary.replace(&issue, &issue.link(opts.issue_link(&issue).unwrap()))
      }
    }
    if let Some(issues) = opts.get_all_pull_requests(&summary) {
      for issue in issues {
        summary = summary.replace(&issue, &issue.link(opts.pull_request_link(&issue).unwrap()))
      }
    }
    format!("{}{} ({})", scope, summary, id)
  }
}
