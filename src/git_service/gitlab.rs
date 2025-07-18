use crate::git_service::{GitService, GitServiceTrait};
use crate::types::{AppError, AppResult, Repository};
use async_trait::async_trait;
use reqwest::Client;
use std::path::Path;

pub struct GitLabService {
    client: Client,
    token: Option<String>,
    git_service: GitService, // Learn: Same composition pattern
    base_url: String,
}

#[async_trait]
impl GitServiceTrait for GitLabService {
    async fn search_repositories(&self, query: &str, page: u32) -> AppResult<Vec<Repository>> {
        todo!("Implement GitLab repository search")
    }

    async fn get_user_repositories(&self, page: u32) -> AppResult<Vec<Repository>> {
        todo!("Implement GitLab user repositories")
    }

    // Learn: Same delegation pattern as GitHub
    async fn clone_repository(&self, repo: &Repository, destination: &Path) -> AppResult<()> {
        self.git_service.clone_repository(repo, destination).await
    }

    fn get_service_name(&self) -> &str {
        "GitLab"
    }

    fn is_authenticated(&self) -> bool {
        self.token.is_some() || self.git_service.ssh_key.is_some()
    }
}
