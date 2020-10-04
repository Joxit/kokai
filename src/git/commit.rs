#[derive(Debug, Clone, Eq)]
pub struct Commit {
  id: String,
  summary: String,
  time: i64,
}

impl Commit {
  fn new(id: String, summary: String, time: i64) -> Self {
    Self { id, summary, time }
  }

  pub fn id(&self) -> &String {
    &self.id
  }

  pub fn summary(&self) -> &String {
    &self.summary
  }
}

impl From<git2::Commit<'_>> for Commit {
  fn from(commit: git2::Commit<'_>) -> Commit {
    Commit::new(
      format!("{}", commit.id()),
      commit.summary().unwrap().to_string(),
      commit.time().seconds(),
    )
  }
}

impl PartialEq for Commit {
  fn eq(&self, other: &Self) -> bool {
    self.id == other.id
  }
}

impl PartialOrd for Commit {
  fn partial_cmp(&self, other: &Commit) -> Option<std::cmp::Ordering> {
    Some(self.cmp(&other))
  }
}

impl Ord for Commit {
  fn cmp(&self, other: &Commit) -> std::cmp::Ordering {
    self
      .time
      .cmp(&other.time)
      .reverse()
      .then(self.id.cmp(&other.id))
  }
}
