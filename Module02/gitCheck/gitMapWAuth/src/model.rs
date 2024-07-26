use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RepoInfo {
    pub name: String,
    pub description: Option<String>,
    pub html_url: String,
    pub stargazers_count: u32,
    pub forks_count: u32,
}

#[derive(Serialize, Deserialize)]
pub struct Credentials {
    pub user: String,
    pub pass: String,
}