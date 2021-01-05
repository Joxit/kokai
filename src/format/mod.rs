pub mod angular;
pub mod formatters;
use regex::Regex;

lazy_static! {
  static ref GITHUB_ISSUES_REGEX: Regex = Regex::new("(^| |\\()(?P<id>#[0-9]+)($| |\\))").unwrap();
}

pub struct FormatOptions {
  pub show_all: bool,
  pub format_url: Option<FormatURL>,
}

pub struct FormatURL {
  pub url: String,
  pub url_format_type: URLFormatTypes,
  pub commits: bool,
  pub issues: bool,
  pub pull_requests: bool,
}

#[derive(PartialEq)]
pub enum URLFormatTypes {
  Github,
  Gitlab,
}

impl FormatOptions {
  pub fn commit_url(&self, id: &String) -> Option<String> {
    if let Some(format_url) = &self.format_url {
      if format_url.commits {
        return Some(format_url.commit(id));
      }
    }
    None
  }

  pub fn get_all_issues(&self, summary: &String) -> Option<Vec<String>> {
    if let Some(format_url) = &self.format_url {
      if format_url.issues {
        let ids = GITHUB_ISSUES_REGEX
          .captures_iter(summary)
          .map(|caps| caps["id"].to_string())
          .filter(|id| id.len() > 0)
          .collect();
        return Some(ids);
      }
    }
    None
  }

  pub fn issue_link(&self, id: &String) -> Option<String> {
    if let Some(format_url) = &self.format_url {
      Some(format_url.issue(id))
    } else {
      None
    }
  }
}

impl FormatURL {
  pub fn new(url: String, add_links: Option<String>) -> Option<FormatURL> {
    let add_links = if let Some(add_links) = add_links {
      add_links
    } else {
      return None;
    };
    let split: Vec<&str> = add_links.splitn(2, ":").collect();
    let url_format_type = match split[0] {
      "github" => URLFormatTypes::Github,
      "gitlab" => URLFormatTypes::Gitlab,
      _ => URLFormatTypes::Github,
    };

    let (commits, issues, pull_requests) = if split.len() > 1 {
      let opts: Vec<&str> = split[1].split(",").collect();
      (
        opts.contains(&"commits"),
        opts.contains(&"issues"),
        opts.contains(&"pr"),
      )
    } else {
      (true, false, false)
    };

    Some(FormatURL {
      url,
      url_format_type,
      commits,
      issues,
      pull_requests,
    })
  }

  pub fn commit(&self, id: &String) -> String {
    match self.url_format_type {
      URLFormatTypes::Github => format!("{}/commit/{}", self.url, id),
      URLFormatTypes::Gitlab => format!("{}/-/commit/{}", self.url, id),
    }
  }

  pub fn issue(&self, id: &String) -> String {
    match self.url_format_type {
      URLFormatTypes::Github => format!("{}/issue/{}", self.url, id),
      URLFormatTypes::Gitlab => format!("{}/-/issues/{}", self.url, id),
    }
  }
}

#[cfg(test)]
mod test {
  use super::*;
  #[test]
  pub fn get_all_issues() {
    for format_type in vec!["github:issues", "gitlab:issues"] {
      let opts = FormatOptions {
        show_all: true,
        format_url: FormatURL::new(
          "git@github.com:joxit/kokai".to_string(),
          Some(format_type.to_string()),
        ),
      };

      assert_eq!(
        opts.get_all_issues(&"#1".to_string()),
        Some(vec!["#1".to_string()])
      );
      assert_eq!(
        opts.get_all_issues(&"#365 foo".to_string()),
        Some(vec!["#365".to_string()])
      );
      assert_eq!(
        opts.get_all_issues(&"foo #35".to_string()),
        Some(vec!["#35".to_string()])
      );
      assert_eq!(
        opts.get_all_issues(&"foo #35 bar".to_string()),
        Some(vec!["#35".to_string()])
      );
      assert_eq!(
        opts.get_all_issues(&"foo (#35) bar".to_string()),
        Some(vec!["#35".to_string()])
      );
      assert_eq!(
        opts.get_all_issues(&"foo (#35) bar (#36)".to_string()),
        Some(vec!["#35".to_string(), "#36".to_string()])
      );
      assert_eq!(
        opts.get_all_issues(&"foo [#35] bar".to_string()),
        Some(vec![])
      );
    }
  }
}
