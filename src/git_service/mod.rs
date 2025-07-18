use crate::types::{AppResult, Repository};
use async_std::path::Path;
use async_trait::async_trait;

#[async_trait]
pub trait GitServiceTrait {
    async fn search_repositories(&self, query: &String, page: u32) -> AppResult<Vec<Repository>>;
    async fn get_user_repositories(&self, page: u32) -> AppResult<Vec<Repository>>;
    async fn clone_repository(&self, repo: &Repository, destination: &Path) -> AppResult<()>;
    fn get_service_name(&self) -> &str;
    fn is_authenticated(&self) -> bool;
}

pub use github::GitHubService;
pub use gitlab::GitLabService;

pub mod github;
pub mod gitlab;
