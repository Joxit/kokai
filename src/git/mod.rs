pub use crate::git::commit::Commit;
use crate::git::traits::{Git2CommitTrait, Git2RepositoryTrait};
use crate::{Error, IntoError};
use chrono::prelude::*;
use git2::Repository;
use std::collections::HashSet;

mod commit;
mod traits;

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
  pub fn repository(&self) -> Result<Repository, Error> {
    Repository::discover(&self.repository).into_error()
  }

  pub fn get_all_commits_before(&self, from: &String) -> Result<Vec<Commit>, Error> {
    let repo = self.repository()?;
    let mut walk = repo.revwalk().into_error()?;
    let start_commit = repo.get_commit_from_ref(&from)?;

    walk.push(start_commit.id()).into_error()?;
    walk.set_sorting(git2::Sort::TOPOLOGICAL).into_error()?;

    Ok(
      walk
        .into_iter()
        .map(|c| c.unwrap())
        .map(|c| repo.revparse_single(&c.to_string()).unwrap())
        .map(|c| c.peel_to_commit().unwrap())
        .map(Commit::from)
        .collect(),
    )
  }

  pub fn get_tag_of(&self, from: &String) -> Result<String, Error> {
    let repo = self.repository()?;
    let from_commit = repo.get_commit_from_ref(&from)?.id_as_string();
    let tag = self
      .get_all_tags()?
      .iter()
      .find(|(commit, _)| commit == &from_commit)
      .map(|(_, tag)| tag.clone());

    if let Some(tag) = tag {
      Ok(tag)
    } else {
      Err(Error::new(format!(
        "No tag found for the reference {}",
        from
      )))
    }
  }

  pub fn get_commit_date(&self, from: &String) -> Result<String, Error> {
    let repo = self.repository()?;
    let start_commit = repo.get_commit_from_ref(&from)?;

    let datetime = FixedOffset::east(60 * start_commit.time().offset_minutes())
      .timestamp(start_commit.time().seconds(), 0);
    Ok(format!("{}", datetime.format("%Y-%m-%d")))
  }

  pub fn get_all_commits_until_tag(&self, from: &String) -> Result<Vec<Commit>, Error> {
    let repo = self.repository()?;
    let mut walk = repo.revwalk().into_error()?;

    let start_commit = repo.get_commit_from_ref(&from)?;

    let all_tags = self.get_all_tags()?;
    let tag_ids: HashSet<&String> = all_tags.iter().map(|(id, _)| id).collect();

    walk.push(start_commit.id())?;
    walk.set_sorting(git2::Sort::TOPOLOGICAL)?;

    let mut result = vec![];
    for oid in walk.into_iter() {
      let object = repo
        .revparse_single(&oid.into_error()?.to_string())
        .into_error()?;
      if let Ok(commit) = object.clone().peel_to_commit() {
        if result.len() != 0 && tag_ids.contains(&commit.id_as_string()) {
          return Ok(result);
        }
        result.push(Commit::from(commit));
      }
    }
    Ok(result)
  }

  pub fn get_all_tags(&self) -> Result<Vec<(String, String)>, Error> {
    let mut res = vec![];
    // Equivalent to self.repository().unwrap().references() with ref filter
    self
      .repository()?
      .tag_foreach(|id, name| {
        res.push((id.to_string(), remove_ref_tags(name)));
        true
      })
      .into_error()?;
    Ok(res)
  }

  pub fn url(&self) -> Result<String, Error> {
    let config = self.repository()?.config()?;

    for entry in &config.entries(Some("url"))? {
      let entry = entry?;
      if let Some(url) = entry.value() {
        return Ok(format!("{}", format_git_url(url)));
      }
    }
    Err(Error::new("URL for your project not found."))
  }
}

fn remove_ref_tags(name: &[u8]) -> String {
  std::str::from_utf8(name)
    .unwrap()
    .chars()
    .skip(10)
    .collect::<String>()
}

fn format_git_url<S: std::string::ToString>(url: S) -> String {
  let url = url.to_string();
  let url = url.strip_suffix(".git").unwrap_or(&url).to_string();
  if url.starts_with("git@") {
    url.replacen(":", "/", 1).replacen("git@", "https://", 1)
  } else {
    url
  }
}

#[cfg(test)]
mod test {
  #[test]
  pub fn format_git_url() {
    assert_eq!(
      super::format_git_url("https://example.com/user/project"),
      "https://example.com/user/project"
    );
    assert_eq!(
      super::format_git_url("https://example.com/user/project.git"),
      "https://example.com/user/project"
    );
    assert_eq!(
      super::format_git_url("git@example.com:user/project"),
      "https://example.com/user/project"
    );
    assert_eq!(
      super::format_git_url("git@example.com:user/project.git"),
      "https://example.com/user/project"
    );
  }
}
