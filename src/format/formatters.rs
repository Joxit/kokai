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
  fn list(&self) -> String
  where
    Self: std::fmt::Display,
  {
    self.list_n(0)
  }
  fn list_n(&self, spaces: usize) -> String
  where
    Self: std::fmt::Display,
  {
    format!("{}* {}", "  ".repeat(spaces), self)
  }
  fn h1(&self) -> String
  where
    Self: std::fmt::Display,
  {
    self.h_n(1)
  }
  fn h3(&self) -> String
  where
    Self: std::fmt::Display,
  {
    self.h_n(3)
  }
  fn h_n(&self, spaces: usize) -> String
  where
    Self: std::fmt::Display,
  {
    format!("{} {}", "#".repeat(spaces), self)
  }
  fn link<S: std::string::ToString>(&self, url: S) -> String
  where
    Self: std::fmt::Display,
  {
    format!("[{}]({})", self, url.to_string())
  }
}

impl Markdown for String {
  fn markdown(&self, _opts: &FormatOptions) -> String {
    self.to_string()
  }
}

impl Markdown for str {
  fn markdown(&self, _opts: &FormatOptions) -> String {
    self.to_string()
  }
}
