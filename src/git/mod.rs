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
    let repo = self.repository();
    let mut walk = repo.revwalk().unwrap();

    let start_commit = repo
      .revparse_single(&from)
      .unwrap()
      .peel_to_commit()
      .unwrap();

    walk.push(start_commit.id()).unwrap();
    walk.set_sorting(git2::Sort::TOPOLOGICAL).unwrap();

    walk
      .into_iter()
      .map(|c| c.unwrap())
      .map(|c| repo.revparse_single(format!("{}", c).as_str()).unwrap())
      .map(|c| c.peel_to_commit().unwrap())
      .map(Commit::from)
      .collect()
  }

  pub fn get_all_tags(&self) -> Vec<String> {
    let mut res = vec![];
    // Equivalent to self.repository().references() with ref filter
    self
      .repository()
      .tag_foreach(|_, name| {
        res.push(remove_ref_tags(name));
        true
      })
      .unwrap();
    res
  }
}

fn remove_ref_tags(name: &[u8]) -> String {
  std::str::from_utf8(name)
    .unwrap()
    .chars()
    .skip(10)
    .collect::<String>()
}

fn commit_walk(commit: &git2::Commit, result: &mut BTreeSet<Commit>) {
  if !result.insert(Commit::from(commit.clone())) {
    return;
  }

  for c in commit.parents() {
    commit_walk(&c, result);
  }
}
