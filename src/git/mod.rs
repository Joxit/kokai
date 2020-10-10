pub use crate::git::commit::Commit;
use git2::Repository;
use std::collections::BTreeSet;

mod commit;

#[derive(Debug, Clone)]
pub struct Git {
  repository: String,
}

impl Git {
  pub fn new(repository: &String) -> Self {
    Self {
      repository: repository.clone(),
    }
  }
  pub fn repository(&self) -> Repository {
    Repository::discover(&self.repository).unwrap()
  }

  pub fn get_all_commits_before(&self, from: &String) -> Vec<Commit> {
    let repository = self.repository();

    let start_commit = repository
      .revparse_single(&from)
      .unwrap()
      .peel_to_commit()
      .unwrap();
    let mut result = BTreeSet::new();
    commit_walk(&start_commit, &mut result);
    result.into_iter().collect::<Vec<Commit>>()
  }

  pub fn get_all_tags(&self) -> Vec<String> {
    let mut res = vec![];
    // Equivalent to self.repository().references() with ref filter
    self.repository().tag_foreach(|_, name| {
      res.push(std::str::from_utf8(name).unwrap().chars().skip(10).collect::<String>());
      true
    }).unwrap();
    res
  }
}

fn commit_walk(commit: &git2::Commit, result: &mut BTreeSet<Commit>) {
  if !result.insert(Commit::from(commit.clone())) {
    return;
  }

  for c in commit.parents() {
    commit_walk(&c, result);
  }
}
