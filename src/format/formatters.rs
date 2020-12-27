use crate::format::FormatOptions;

pub trait Markdown {
  fn markdown(&self, opts: &FormatOptions) -> String;
}
