use crate::git_service::GitServiceTrait;
use crate::types::{AppError, AppResult, Repository};
use async_trait::async_trait;
use reqwest::Client;
use reqwest::Response;
use serde::{Deserialize, Serialize};
use std::path::Path;

pub struct GitHubService {
    client: Client,
    token: Option<String>,
    ssh_key: Option<String>,
}

#[derive(Debug, Deserialize)]
struct GitHubRepo {
    id: u64,
    name: String,
    full_name: String,
    description: Option<String>,
    clone_url: String,
    ssh_url: String,
    stargazers_count: u32,
    forks_count: u32,
    private: bool,
}

#[derive(Debug, Deserialize)]
struct GitHubSearchResponse {
    items: Vec<GitHubRepo>,
    total_count: u32,
}

impl GitHubService {
    async fn make_request(&self, url: &str) -> AppResult<reqwest::Response> {
        let mut request = self.client.get(url);

        if let Some(token) = &self.token {
            request = request.header("Authorization", format!("Bearer {}", token))
        }

        request
            .header("User-Agent", "git-tui-cloner/0.1.0")
            .header("Accept", "application/vnd.github+json")
            .header("X-GitHub-Api-Version", "2022-11-28")
            .send()
            .await
            .map_err(|e| AppError::NetworkError(format!("Request failed: {}", e)))
    }
}

#[async_trait]
impl GitServiceTrait for GitHubService {
    async fn search_repositories(&self, query: &str, page: u32) -> AppResult<Vec<Repository>> {
        if self.token.is_none() {
            return Err(AppError::ServiceWarning(
                "GitHub: To prevent hitting rate limits, please provide a GitHub token".to_string(),
            ));
        };

        let url = format!(
            "https://api.github.com/search/repositories?q={}&page={}&per_page=30",
            urlencoding::encode(query),
            page
        );

        let response = self.make_request(&url).await?;

        if !response.status().is_success() {
            return Err(AppError::NetworkError(format!(
                "GitHub API Request Failed: {}",
                response.status()
            )));
        }

        let search_response: GitHubSearchResponse = response.json().await.map_err(|e| {
            AppError::NetworkError(format!("Failed to parse GitHub API response: {}", e))
        })?;

        Ok(search_response
            .items
            .into_iter()
            .map(|repo| Repository {
                id: repo.id.to_string(),
                name: repo.name,
                full_name: repo.full_name,
                description: repo.description,
                clone_url: repo.clone_url,
                ssh_url: repo.ssh_url,
                stars: repo.stargazers_count,
                forks: repo.forks_count,
                private: repo.private,
                clone_type: None,
            })
            .collect());
    }

    async fn get_user_repositories(&self, page: u32) -> AppResult<Vec<Repository>> {
        if self.token.is_none() {
            return Err(AppError::ServiceWarning(
                "GitHub: To prevent hitting rate limits, please provide a GitHub token".to_string(),
            ));
        }

        let url = format!(
            "https://api.github.com/user/repos?page={}&per_page=30",
            page
        );
        let response = self.make_request(&url).await?;
        if !response.status().is_success() {
            return Err(AppError::NetworkError(format!(
                "GitHub API Request Failed: {}",
                response.status()
            )));
        }
        let repos: Vec<GitHubRepo> = response.json().await.map_err(|e| {
            AppError::NetworkError(format!("Failed to parse GitHub API response: {}", e))
        })?;

        Ok(repos
            .into_iter()
            .map(|repo| Repository {
                id: repo.id.to_string(),
                name: repo.name,
                full_name: repo.full_name,
                description: repo.description,
                clone_url: repo.clone_url,
                ssh_url: repo.ssh_url,
                stars: repo.stargazers_count,
                forks: repo.forks_count,
                private: repo.private,
                clone_type: None,
            })
            .collect())
    }

    async fn clone_repository(&self, repo: &Repository, destination: &Path) -> AppResult<()> {
        todo!("Implement GitHub clone repository")
    }

    fn get_service_name(&self) -> &str {
        "GitHub"
    }

    fn is_authenticated(&self) -> bool {
        self.token.is_some() || self.ssh_key.is_some()
    }
}
