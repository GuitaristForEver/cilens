/// A GitHub Actions workflow run.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GitHubWorkflowRun {
    /// Workflow run ID
    pub id: u64,
    /// Git reference (branch/tag)
    pub head_branch: Option<String>,
    /// Trigger event (push, pull_request, schedule, etc.)
    pub event: String,
    /// Final status (completed, in_progress, queued)
    pub status: String,
    /// Conclusion (success, failure, cancelled, skipped, etc.)
    pub conclusion: Option<String>,
    /// Duration in milliseconds
    pub run_duration_ms: Option<u64>,
    /// All jobs in this workflow run
    pub jobs: Vec<GitHubJob>,
}

/// A job within a GitHub Actions workflow run.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GitHubJob {
    /// Job ID
    pub id: u64,
    /// Job name
    pub name: String,
    /// Final status (completed, in_progress, queued)
    pub status: String,
    /// Conclusion (success, failure, cancelled, skipped, etc.)
    pub conclusion: Option<String>,
    /// Started at timestamp
    pub started_at: Option<String>,
    /// Completed at timestamp
    pub completed_at: Option<String>,
    /// Job dependencies via `needs`
    pub needs: Option<Vec<String>>,
}
