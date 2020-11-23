use git2::{Commit, Repository};
pub trait Git2RepositoryTrait {
  fn get_commit_from_ref(&self, commit: &String) -> Commit;
}

impl Git2RepositoryTrait for Repository {
  fn get_commit_from_ref(&self, commit: &String) -> Commit {
    self
      .revparse_single(&commit)
      .unwrap()
      .peel_to_commit()
      .unwrap()
  }
}

pub trait Git2CommitTrait {
  fn id_as_string(&self) -> String;
}

impl Git2CommitTrait for Commit<'_> {
  fn id_as_string(&self) -> String {
    self.id().to_string()
  }
}
