use crate::format::formatters::Markdown;
use crate::format::FormatOptions;
use crate::parser::{ConventionalCommit as CC, ConventionalCommitType as CCT};
use std::io::Write;

pub fn print_conventional_commit_release<W: Write>(
  w: &mut W,
  tag: &String,
  date: Option<String>,
  commits: &Vec<CC>,
  opts: FormatOptions,
) -> std::io::Result<()> {
  let fix: Vec<&CC> = commits
    .iter()
    .filter(|&c| c.commit_type == CCT::Fix)
    .collect();
  let feat: Vec<&CC> = commits
    .iter()
    .filter(|c| c.commit_type == CCT::Feature)
    .collect();
  let perf: Vec<&CC> = commits
    .iter()
    .filter(|c| c.commit_type == CCT::Perf)
    .collect();
  let revert: Vec<&CC> = commits
    .iter()
    .filter(|c| c.commit_type == CCT::Revert)
    .collect();
  let build: Vec<&CC> = commits
    .iter()
    .filter(|c| c.commit_type == CCT::Build)
    .collect();
  let chore: Vec<&CC> = commits
    .iter()
    .filter(|c| c.commit_type == CCT::Chore)
    .collect();
  let ci: Vec<&CC> = commits
    .iter()
    .filter(|c| c.commit_type == CCT::Ci)
    .collect();
  let docs: Vec<&CC> = commits
    .iter()
    .filter(|c| c.commit_type == CCT::Docs)
    .collect();
  let style: Vec<&CC> = commits
    .iter()
    .filter(|c| c.commit_type == CCT::Style)
    .collect();
  let refactor: Vec<&CC> = commits
    .iter()
    .filter(|c| c.commit_type == CCT::Refactor)
    .collect();
  let test: Vec<&CC> = commits
    .iter()
    .filter(|c| c.commit_type == CCT::Test)
    .collect();

  if let Some(date) = date {
    writeln!(w, "{} ({})", tag.h1(), date)?;
  } else {
    writeln!(w, "{}", tag.h1())?;
  }

  if !fix.is_empty() {
    writeln!(w, "\n{}\n", ":beetle: Bug Fixes".h3())?;
    for c in &fix {
      writeln!(w, "{}", c.markdown(&opts).list())?;
    }
  }
  if !refactor.is_empty() {
    writeln!(w, "\n{}\n", ":ghost: Code Refactoring".h3())?;
    for c in &refactor {
      writeln!(w, "{}", c.markdown(&opts).list())?;
    }
  }
  if !feat.is_empty() {
    writeln!(w, "\n{}\n", ":sparkles: Features".h3())?;
    for c in &feat {
      writeln!(w, "{}", c.markdown(&opts).list())?;
    }
  }
  if !build.is_empty() {
    writeln!(w, "\n{}\n", ":wrench: Build".h3())?;
    for c in &build {
      writeln!(w, "{}", c.markdown(&opts).list())?;
    }
  }
  if !perf.is_empty() {
    writeln!(w, "\n{}\n", ":racehorse: Performance Improvements".h3())?;
    for c in &perf {
      writeln!(w, "{}", c.markdown(&opts).list())?;
    }
  }
  if !revert.is_empty() {
    writeln!(w, "\n{}\n", ":arrow_backward: Reverts".h3())?;
    for c in &revert {
      writeln!(w, "{}", c.markdown(&opts).list())?;
    }
  }
  if !test.is_empty() && opts.show_all {
    writeln!(w, "\n{}\n", ":heavy_check_mark: Tests".h3())?;
    for c in &test {
      writeln!(w, "{}", c.markdown(&opts).list())?;
    }
  }
  if !style.is_empty() && opts.show_all {
    writeln!(w, "\n{}\n", ":art: Style Changes".h3())?;
    for c in &style {
      writeln!(w, "{}", c.markdown(&opts).list())?;
    }
  }
  if !docs.is_empty() && opts.show_all {
    writeln!(w, "\n{}\n", ":memo: Documentation".h3())?;
    for c in &docs {
      writeln!(w, "{}", c.markdown(&opts).list())?;
    }
  }
  if !ci.is_empty() && opts.show_all {
    writeln!(w, "\n{}\n", ":rocket: Continuous Integration".h3())?;
    for c in &ci {
      writeln!(w, "{}", c.markdown(&opts).list())?;
    }
  }
  if !chore.is_empty() && opts.show_all {
    writeln!(w, "\n{}\n", ":green_apple: Chore".h3())?;
    for c in &chore {
      writeln!(w, "{}", c.markdown(&opts).list())?;
    }
  }

  writeln!(w, "")
}
