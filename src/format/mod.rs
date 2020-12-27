pub mod angular;
pub mod formatters;

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
}
