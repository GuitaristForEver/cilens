mod provider;
mod types;
pub mod client;

pub use provider::GitHubProvider;
pub use types::{GitHubWorkflowRun, GitHubJob};
pub use client::{GitHubClient, WorkflowRunData};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_github_provider_exists() {
        let _: Option<GitHubProvider> = None;
    }
}
