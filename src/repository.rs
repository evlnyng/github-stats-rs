//! For getting repository information.

use chrono::prelude::{DateTime, Utc};
use serde::Deserialize;

use crate::{Result, User};

/// Represents that stats of a [Github] repository.
///
/// [Github]: https://github.com/
#[derive(Debug, Deserialize)]
pub struct Repo {
    id: u64,
    node_id: String,
    name: String,
    full_name: String,
    private: bool,
    owner: User,
    html_url: String,
    description: String,
    fork: bool,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    pushed_at: DateTime<Utc>,
    homepage: String,
    /// In *kilo*bytes.
    size: u64,
    stargazers_count: u64,
    language: String,
    forks_count: u64,
    archived: bool,
    disabled: bool,
    /// Issues + PRs
    open_issues: u64,
    default_branch: String,
    /// Number of watchers.
    subscribers_count: u64,
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
        let repo: Repo = reqwest::get(&repo_api_url(user, repo))?.json()?;

        Ok(repo)
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn node_id(&self) -> &str {
        &self.node_id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn full_name(&self) -> &str {
        &self.full_name
    }

    pub fn private(&self) -> bool {
        self.private
    }

    pub fn owner(&self) -> &User {
        &self.owner
    }

    pub fn html_url(&self) -> &str {
        &self.html_url
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn fork(&self) -> bool {
        self.fork
    }

    pub fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }

    pub fn updated_at(&self) -> &DateTime<Utc> {
        &self.updated_at
    }

    pub fn pushed_at(&self) -> &DateTime<Utc> {
        &self.pushed_at
    }

    pub fn homepage(&self) -> &str {
        &self.homepage
    }

    /// In *kilo*bytes.
    pub fn size(&self) -> u64 {
        self.size
    }

    pub fn stargazers_count(&self) -> u64 {
        self.stargazers_count
    }

    pub fn language(&self) -> &str {
        &self.language
    }

    pub fn forks_count(&self) -> u64 {
        self.forks_count
    }

    pub fn archived(&self) -> bool {
        self.archived
    }

    pub fn disabled(&self) -> bool {
        self.disabled
    }

    /// Issues + PRs
    pub fn open_issues(&self) -> u64 {
        self.open_issues
    }

    pub fn default_branch(&self) -> &str {
        &self.default_branch
    }

    /// Number of watchers.
    pub fn subscribers_count(&self) -> u64 {
        self.subscribers_count
    }
}

// Takes [Github] user and repo IDs to make a link to the API for that repo.
//
// [Github]: https://github.com/
fn repo_api_url(user: &str, repo: &str) -> String {
    const URL: &str = "https://api.github.com/repos";
    format!("{}/{}/{}", URL, user, repo)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        println!("{:#?}", Repo::new("rust-lang", "rust").unwrap());
        assert!(true);
    }
}
