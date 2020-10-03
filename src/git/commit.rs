#[derive(Debug, Clone)]
pub struct Commit {
  id: String,
  summary: String,
}

impl Commit {
  fn new(id: String, summary: String) -> Self {
    Self { id, summary }
  }

  fn id(&self) -> &String {
    &self.id
  }

  fn summary(&self) -> &String {
    &self.summary
  }
}

impl From<git2::Commit<'_>> for Commit {
  fn from(commit: git2::Commit<'_>) -> Commit {
    Commit::new(
      format!("{}", commit.id()),
      commit.summary().unwrap().to_string(),
    )
  }
}

impl PartialEq for Commit {
  fn eq(&self, other: &Self) -> bool {
    self.id == other.id
  }
}
