use crate::auth::Token;
use crate::error::Result;

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
