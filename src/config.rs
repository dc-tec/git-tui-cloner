use crate::types::{AppError, AppResult};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub github_token: Option<String>,
    pub gitlab_token: Option<String>,
    pub ssh_key: Option<String>,
    pub gitlab_instance: Vec<GitLabInstance>,
    pub default_clone_path: PathBuf,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GitLabInstance {
    pub name: String,
    pub url: String,
    pub token: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            github_token: None,
            gitlab_token: None,
            ssh_key: None,
            default_clone_path: dirs::home_dir()
                .map(|home| home.join("projects"))
                .unwrap_or_else(|| PathBuf::from("./projects")),
            gitlab_instance: vec![GitLabInstance {
                name: "GitLab".to_string(),
                url: "https://gitlab.com".to_string(),
                token: None,
            }],
        }
    }
}

impl Config {
    pub fn load() -> AppResult<Self> {
        let config_path = Self::config_path()?;

        if config_path.exists() {
            let content = std::fs::read_to_string(&config_path)
                .map_err(|e| AppError::ConfigError(format!("Failed to read config file: {}", e)))?;

            serde_json::from_str(&content)
                .map_err(|e| AppError::ConfigError(format!("Failed to parse config file: {}", e)))
        } else {
            Ok(Self::default())
        }
    }

    pub fn save(&self) -> AppResult<()> {
        let config_path = Self::config_path()?;

        if let Some(parent) = config_path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| {
                AppError::ConfigError(format!("Failed to create config directory: {}", e))
            })?;
        }

        let content = serde_json::to_string_pretty(self)
            .map_err(|e| AppError::ConfigError(format!("Failed to serialize config: {}", e)))?;

        std::fs::write(&config_path, content)
            .map_err(|e| AppError::ConfigError(format!("Failed to write config file: {}", e)))?;

        Ok(())
    }

    fn config_path() -> AppResult<PathBuf> {
        dirs::home_dir()
            .map(|mut path| {
                path.push(".config");
                path.push("git-tui-cloner");
                path.push("config.json");
                path
            })
            .ok_or_else(|| AppError::ConfigError("Failed to determine home directory".to_string()))
    }
}
