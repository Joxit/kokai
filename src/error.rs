#[derive(Debug, Clone)]
pub struct Error {
  message: String,
}

pub trait IntoError<T> {
  fn into_error(self) -> Result<T, Error>;
}

impl Error {
  pub fn new<S: std::string::ToString>(message: S) -> Self {
    Error {
      message: message.to_string(),
    }
  }
  pub fn message(&self) -> &String {
    &self.message
  }
}

impl From<git2::Error> for Error {
  fn from(error: git2::Error) -> Error {
    Error {
      message: format!("Error with your git repository: {}", error.message()),
    }
  }
}

impl<T> IntoError<T> for Result<T, git2::Error> {
  fn into_error(self) -> Result<T, Error> {
    self.map_err(|error| Error {
      message: format!("Error with your git repository: {}", error.message()),
    })
  }
}
