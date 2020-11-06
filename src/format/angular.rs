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
    writeln!(w, "\n### Bug Fixes\n")?;
    for c in &fix {
      writeln!(w, "* {}", c)?;
    }
  }
  if !refactor.is_empty() {
    writeln!(w, "\n### Code Refactoring\n")?;
    for c in &refactor {
      writeln!(w, "* {}", c)?;
    }
  }
  if !feat.is_empty() {
    writeln!(w, "\n### Features\n")?;
    for c in &feat {
      writeln!(w, "* {}", c)?;
    }
  }
  if !build.is_empty() {
    writeln!(w, "\n### Build\n")?;
    for c in &build {
      writeln!(w, "* {}", c)?;
    }
  }
  if !perf.is_empty() {
    writeln!(w, "\n### Performance Improvements\n")?;
    for c in &perf {
      writeln!(w, "* {}", c)?;
    }
  }
  if !revert.is_empty() {
    writeln!(w, "\n### Reverts\n")?;
    for c in &revert {
      writeln!(w, "* {}", c)?;
    }
  }
  if !test.is_empty() && opts.show_all {
    writeln!(w, "\n### Tests\n")?;
    for c in &test {
      writeln!(w, "* {}", c)?;
    }
  }
  if !style.is_empty() && opts.show_all {
    writeln!(w, "\n### Style Changes\n")?;
    for c in &style {
      writeln!(w, "* {}", c)?;
    }
  }
  if !docs.is_empty() && opts.show_all {
    writeln!(w, "\n### Documentation\n")?;
    for c in &docs {
      writeln!(w, "* {}", c)?;
    }
  }
  if !ci.is_empty() && opts.show_all {
    writeln!(w, "\n### Continuous Integration\n")?;
    for c in &ci {
      writeln!(w, "* {}", c)?;
    }
  }
  if !chore.is_empty() && opts.show_all {
    writeln!(w, "\n### Chore\n")?;
    for c in &chore {
      writeln!(w, "* {}", c)?;
    }
  }

  writeln!(w, "")
}
