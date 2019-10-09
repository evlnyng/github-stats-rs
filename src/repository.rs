use big_bytes::BigByte;

use crate::{JsonMap, Response, Result};

use issues::issue_stats;
pub use releases::Release;

mod issues;
pub mod languages;
pub mod releases;

// The URL for [Github] repository data.
//
// Append `/`*user*`/`*repo* to the end for the full URL.
//
// # Example
//
// ```
// let url = format!("{}/{}/{}", GITHUB_API_REPO_URL, "rust-lang", "rust");
// ```
//
// [Github]: https://github.com/
const GITHUB_API_REPO_URL: &str = "https://api.github.com/repos";

/// Represents that stats of a [Github] repository.
///
/// [Github]: https://github.com/
#[derive(Debug)]
pub struct Repo {
    name: String,
    created: String,
    updated: String,
    primary_language: String,
    languages: JsonMap<u64>,
    homepage: Option<String>,
    size: f64,
    stars: u64,
    forks: u64,
    open_issues: u64,
    closed_issues: u64,
    open_pull_requests: u64,
    closed_pull_requests: u64,
    latest_release: Option<Release>,
    is_fork: bool,
}

impl Repo {
    /// Creates a new `Repo`.
    ///
    /// # Example
    ///
    /// ```
    /// use github_stats::Repo;
    ///
    /// let repo = Repo::new("rust-lang", "rust");
    /// ```
    pub fn new(user: &str, repo: &str) -> Result<Self> {
        let repo_data = repo_stats(user, repo)?;
        let name = repo_data["name"]
            .as_str()
            .ok_or(r#""name" is not a string"#)?
            .to_string();
        let created = repo_data["created_at"]
            .as_str()
            .ok_or(r#""name" is not a string"#)?
            .to_string();
        let updated = repo_data["updated_at"]
            .as_str()
            .ok_or(r#""name" is not a string"#)?
            .to_string();
        let primary_language = repo_data["language"]
            .as_str()
            .ok_or(r#""language" is not a string"#)?
            .to_string();
        let languages = languages::from_api_url(
            repo_data["languages_url"]
            .as_str()
            .ok_or(r#""languages_url" is not a string"#)?
        )?;
        let homepage = match repo_data["homepage"].as_str() {
            None | Some("") => None,
            Some(s) => Some(String::from(s)),

        };
        let size = repo_data["size"]
            .as_f64()
            .ok_or(r#""size" cannot be read as f64"#)?;
        let stars = repo_data["stargazers_count"]
            .as_u64()
            .ok_or(r#""stars" cannot be read as u64"#)?;
        let forks = repo_data["forks"]
            .as_u64()
            .ok_or(r#""forks_count" cannot be read as u64"#)?;
        let (open_issues, closed_issues, open_pull_requests, closed_pull_requests) =
            issue_stats(user, repo)?;
        let is_fork = repo_data["fork"]
            .as_bool()
            .ok_or(r#""fork" could not be read as bool"#)?;
        let latest_release = Release::latest(user, repo)?;

        let repo = Repo {
            name,
            created,
            updated,
            primary_language,
            languages,
            homepage,
            size,
            stars,
            forks,
            open_issues,
            closed_issues,
            open_pull_requests,
            closed_pull_requests,
            latest_release,
            is_fork,
        };
        Ok(repo)
    }

    /// Gets the repository's name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Gets the repository's creation date.
    pub fn created(&self) -> &str {
        &self.created
    }

    /// Gets the date of the repository's latest updated.
    pub fn updated(&self) -> &str {
        &self.updated
    }

    /// Repository's primary language.
    pub fn primary_language(&self) -> &str {
        &self.primary_language
    }

    /// All languages in this repository.
    ///
    /// Maps language name to number of bytes of code in that language.
    pub fn languages(&self) -> &JsonMap<u64> {
        &self.languages
    }

    /// The repository's homepage, if it exists.
    pub fn homepage(&self) -> &Option<String> {
        &self.homepage
    }

    /// Gets the repository's size in kilobytes.
    pub fn size(&self) -> f64 {
        self.size
    }

    /// Gets the repository's size in a human-readable format.
    ///
    /// `precision` is the number of decimal places to display.
    pub fn human_size(&self, precision: usize) -> String {
        (self.size * 1024_f64).big_byte(precision)
    }

    /// Gets the repository's star count.
    pub fn stars(&self) -> u64 {
        self.stars
    }

    /// Gets the repository's fork count.
    pub fn forks(&self) -> u64 {
        self.forks
    }

    /// Gets the repository's open issue count.
    pub fn open_issues(&self) -> u64 {
        self.open_issues
    }

    /// Gets the repository's closed issue count.
    pub fn closed_issues(&self) -> u64 {
        self.closed_issues
    }

    /// Gets the repository's open pull request count.
    pub fn open_pull_requests(&self) -> u64 {
        self.open_pull_requests
    }

    /// Gets the repository's closed pull request count.
    pub fn closed_pull_requests(&self) -> u64 {
        self.closed_pull_requests
    }

    /// The latest release.
    pub fn latest_release(&self) -> &Option<Release> {
        &self.latest_release
    }

    /// If this repository is a fork.
    pub fn is_fork(&self) -> bool {
        self.is_fork
    }
}

// Takes [Github] user and repo IDs to make a link to the API for that repo.
//
// [Github]: https://github.com/
fn repo_api_url(user: &str, repo: &str) -> String {
    format!("{}/{}/{}", GITHUB_API_REPO_URL, user, repo)
}

// Requests repo data from [Github]'s API.
//
// [Github]: https://github.com/
fn repo_stats(user: &str, repo: &str) -> Result<Response> {
    let response: Response = reqwest::get(&repo_api_url(user, repo))?.json()?;
    Ok(response)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        println!("{:#?}", repo_stats("rust-lang", "rust").unwrap());
        assert!(true);
    }
}