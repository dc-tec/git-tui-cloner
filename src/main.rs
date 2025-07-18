mod config;
mod git_service;
mod types;

use config::Config;
use git_service::{GitHubService, GitLabService};
use types::AppResult;

fn main() -> AppResult<()> {
    println!("Starting Git TUI Cloner");

    let config = Config::load()?;
    println!("Loaded config");

    println!(
        "Default clone path: {:?}",
        config.default_clone_path.display()
    );
    println!("GitLab instances: {:?}", config.gitlab_instance);
    println!("GitHub token: {}", config.github_token.is_some());
    println!("GitLab token: {}", config.gitlab_token.is_some());

    config.save()?;
    println!("Config saved");

    Ok(())
}
