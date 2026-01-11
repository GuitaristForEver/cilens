use crate::auth::Token;
use crate::error::Result;
use chrono::{DateTime, Utc};

/// GitHub Actions CI/CD insights provider.
pub struct GitHubProvider {
    pub base_url: String,
    pub owner: String,
    pub repo: String,
    pub token: Option<Token>,
}

impl GitHubProvider {
    /// Creates a new GitHub provider.
    pub fn new(
        base_url: &str,
        owner: String,
        repo: String,
        token: Option<Token>,
    ) -> Result<Self> {
        Ok(Self {
            base_url: base_url.to_string(),
            owner,
            repo,
            token,
        })
    }
}

/// Calculates duration between two timestamps in milliseconds.
fn calculate_duration(started_at: Option<&str>, completed_at: Option<&str>) -> Option<u64> {
    match (started_at, completed_at) {
        (Some(start), Some(end)) => {
            let start: DateTime<Utc> = start.parse().ok()?;
            let end: DateTime<Utc> = end.parse().ok()?;
            let duration = end.signed_duration_since(start);
            Some(duration.num_milliseconds() as u64)
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::providers::github::types::GitHubWorkflowRun;

    #[test]
    fn test_calculate_duration() {
        let started = "2025-01-01T00:00:00Z";
        let completed = "2025-01-01T00:05:00Z";

        let duration = calculate_duration(Some(started), Some(completed));
        assert_eq!(duration, Some(300_000)); // 5 minutes in ms
    }

    #[test]
    fn test_calculate_duration_missing_timestamps() {
        assert_eq!(calculate_duration(None, None), None);
        assert_eq!(calculate_duration(Some("2025-01-01T00:00:00Z"), None), None);
    }

    #[test]
    fn test_transform_to_pipeline_format() {
        // Test that GitHubWorkflowRun has fields compatible with pipeline analysis
        let run = GitHubWorkflowRun {
            id: 123,
            head_branch: Some("main".to_string()),
            event: "push".to_string(),
            status: "completed".to_string(),
            conclusion: Some("success".to_string()),
            run_duration_ms: Some(300_000),
            jobs: vec![],
        };

        assert_eq!(run.id, 123);
        assert_eq!(run.head_branch, Some("main".to_string()));
        assert!(run.is_completed());
    }
}
