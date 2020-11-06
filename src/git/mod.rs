pub use crate::git::commit::Commit;
use chrono::prelude::*;
use git2::Repository;
use std::collections::HashSet;

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

  pub fn get_commit_date(&self, from: &String) -> String {
    let repo = self.repository();
    let start_commit = repo
      .revparse_single(&from)
      .unwrap()
      .peel_to_commit()
      .unwrap();

    let datetime = FixedOffset::east(60 * start_commit.time().offset_minutes())
      .timestamp(start_commit.time().seconds(), 0);
    format!("{}", datetime.format("%Y-%m-%d"))
  }

  pub fn get_all_commits_until_tag(&self, from: &String) -> Vec<Commit> {
    let repo = self.repository();
    let mut walk = repo.revwalk().unwrap();

    let start_commit = repo
      .revparse_single(&from)
      .unwrap()
      .peel_to_commit()
      .unwrap();
    let all_tags = self.get_all_tags();
    let tag_ids: HashSet<&String> = all_tags.iter().map(|(id, _)| id).collect();

    walk.push(start_commit.id()).unwrap();
    walk.set_sorting(git2::Sort::TOPOLOGICAL).unwrap();

    let iter = walk
      .into_iter()
      .map(|c| c.unwrap())
      .map(|c| repo.revparse_single(format!("{}", c).as_str()).unwrap());

    let mut result = vec![];
    for object in iter {
      if let Ok(commit) = object.clone().peel_to_commit() {
        if result.len() != 0 && tag_ids.contains(&format!("{}", commit.id())) {
          return result;
        }
        result.push(Commit::from(commit));
      }
    }
    result
  }

  pub fn get_all_tags(&self) -> Vec<(String, String)> {
    let mut res = vec![];
    // Equivalent to self.repository().references() with ref filter
    self
      .repository()
      .tag_foreach(|id, name| {
        res.push((format!("{}", id), remove_ref_tags(name)));
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
