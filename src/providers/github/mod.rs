mod provider;
mod types;
mod cache;
pub mod client;

pub use provider::GitHubProvider;
pub use types::{GitHubWorkflowRun, GitHubJob};
pub use client::{GitHubClient, WorkflowRunData};
pub use cache::JobCache;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_github_provider_exists() {
        let _: Option<GitHubProvider> = None;
    }
}
