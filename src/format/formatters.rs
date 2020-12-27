use crate::format::FormatOptions;

pub trait Markdown {
  fn markdown(&self, opts: &FormatOptions) -> String;
  fn bold(&self) -> String
  where
    Self: std::fmt::Display,
  {
    format!("**{}**", self)
  }
  fn italic(&self) -> String
  where
    Self: std::fmt::Display,
  {
    format!("_{}_", self)
  }
  fn code(&self) -> String
  where
    Self: std::fmt::Display,
  {
    format!("`{}`", self)
  }
  fn strike(&self) -> String
  where
    Self: std::fmt::Display,
  {
    format!("~~{}~~", self)
  }
  fn quote(&self) -> String
  where
    Self: std::fmt::Display,
  {
    format!("> {}", self)
  }
  fn link<S: std::string::ToString>(&self, url: S) -> String
  where
    Self: std::fmt::Display,
  {
    format!("[{}]({})", self, url.to_string())
  }
}

impl Markdown for String {
  fn markdown(&self, opts: &FormatOptions) -> String {
    self.to_string()
  }
}
