#[derive(Debug, Clone)]
pub struct Repository {
    pub id: String,
    pub name: String,
    pub full_name: String,
    pub description: String,
    pub clone_url: String,
    pub ssh_url: String,
    pub stars: u32,
    pub forks: u32,
    pub private: bool,
}

#[derive(Debug)]
pub enum GitService {
    GitHub,
    GitLab,
    SelfHostedGitLab(String),
}

#[derive(Debug)]
pub enum AppError {
    NetworkError(String),
    AuthenticationError(String),
    ConfigError(String),
    GitError(String),
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::NetworkError(e) => write!(f, "Network error: {}", e),
            AppError::AuthenticationError(e) => write!(f, "Authentication error: {}", e),
            AppError::ConfigError(e) => write!(f, "Configuration error: {}", e),
            AppError::GitError(e) => write!(f, "Git error: {}", e),
        }
    }
}

impl std::error::Error for AppError {}

pub type AppResult<T> = Result<T, AppError>;
