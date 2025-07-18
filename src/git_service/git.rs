use crate::types::{AppError, AppResult, AppWarning, CloneType, Repository};
use git2::Repository as GitRepository;
use git2::{Cred, Error, RemoteCallbacks};
use std::env;
use std::path::Path;

pub struct GitService {
    pub ssh_key: Option<String>,
}

impl GitService {
    pub fn new() -> Self {
        Self { ssh_key: None }
    }
    pub fn with_ssh_key(ssh_key: String) -> Self {
        Self {
            ssh_key: Some(ssh_key),
        }
    }

    pub async fn clone_repository(&self, repo: &Repository, destination: &Path) -> AppResult<()> {
        todo!("Implement git clone repository")
    }
}
