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
    writeln!(w, "# {} ({})", tag, date)?;
  } else {
    writeln!(w, "# {}", tag)?;
  }

  if !fix.is_empty() {
    writeln!(w, "\n### :beetle: Bug Fixes\n")?;
    for c in &fix {
      writeln!(w, "* {}", c.markdown())?;
    }
  }
  if !refactor.is_empty() {
    writeln!(w, "\n### :ghost: Code Refactoring\n")?;
    for c in &refactor {
      writeln!(w, "* {}", c.markdown())?;
    }
  }
  if !feat.is_empty() {
    writeln!(w, "\n### :sparkles: Features\n")?;
    for c in &feat {
      writeln!(w, "* {}", c.markdown())?;
    }
  }
  if !build.is_empty() {
    writeln!(w, "\n### :shipit: Build\n")?;
    for c in &build {
      writeln!(w, "* {}", c.markdown())?;
    }
  }
  if !perf.is_empty() {
    writeln!(w, "\n### :racehorse: Performance Improvements\n")?;
    for c in &perf {
      writeln!(w, "* {}", c.markdown())?;
    }
  }
  if !revert.is_empty() {
    writeln!(w, "\n### :arrow_backward: Reverts\n")?;
    for c in &revert {
      writeln!(w, "* {}", c.markdown())?;
    }
  }
  if !test.is_empty() && opts.show_all {
    writeln!(w, "\n### :heavy_check_mark: Tests\n")?;
    for c in &test {
      writeln!(w, "* {}", c.markdown())?;
    }
  }
  if !style.is_empty() && opts.show_all {
    writeln!(w, "\n### :art: Style Changes\n")?;
    for c in &style {
      writeln!(w, "* {}", c.markdown())?;
    }
  }
  if !docs.is_empty() && opts.show_all {
    writeln!(w, "\n### :memo: Documentation\n")?;
    for c in &docs {
      writeln!(w, "* {}", c.markdown())?;
    }
  }
  if !ci.is_empty() && opts.show_all {
    writeln!(w, "\n### :rocket: Continuous Integration\n")?;
    for c in &ci {
      writeln!(w, "* {}", c.markdown())?;
    }
  }
  if !chore.is_empty() && opts.show_all {
    writeln!(w, "\n### :green_apple: Chore\n")?;
    for c in &chore {
      writeln!(w, "* {}", c.markdown())?;
    }
  }

  writeln!(w, "")
}
