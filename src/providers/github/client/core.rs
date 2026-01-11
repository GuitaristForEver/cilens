use crate::auth::Token;
use crate::error::Result;
use reqwest::Client;
use std::time::Duration;
use url::Url;

/// GitHub REST API client.
///
/// Handles HTTP requests to the GitHub API with authentication and retry logic.
pub struct GitHubClient {
    pub base_url: Url,
    pub client: Client,
    pub token: Option<Token>,
}

impl GitHubClient {
    /// Creates a new GitHub API client.
    ///
    /// # Arguments
    /// * `base_url` - The GitHub API base URL (e.g., "https://api.github.com")
    /// * `token` - Optional authentication token
    ///
    /// # Errors
    /// Returns an error if the base URL is invalid or the HTTP client cannot be created.
    pub fn new(base_url: &str, token: Option<Token>) -> Result<Self> {
        let base_url = Url::parse(base_url)
            .map_err(|e| crate::error::CILensError::Config(format!("Invalid base URL: {e}")))?;

        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .map_err(|e| {
                crate::error::CILensError::Config(format!("Failed to create HTTP client: {e}"))
            })?;

        Ok(Self {
            base_url,
            client,
            token,
        })
    }

    /// Returns the authorization header value if a token is present.
    ///
    /// Returns `Some("Bearer {token}")` if a token is configured, `None` otherwise.
    fn auth_header(&self) -> Option<String> {
        self.token
            .as_ref()
            .map(|token| format!("Bearer {}", token.as_str()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let result = GitHubClient::new("https://api.github.com", None);

        assert!(result.is_ok());
        let client = result.unwrap();
        assert_eq!(client.base_url.as_str(), "https://api.github.com/");
        assert!(client.token.is_none());
    }

    #[test]
    fn test_client_with_token() {
        let token = Token::from("ghp_test");
        let result = GitHubClient::new("https://api.github.com", Some(token));

        assert!(result.is_ok());
        let client = result.unwrap();
        assert_eq!(client.base_url.as_str(), "https://api.github.com/");
        assert!(client.token.is_some());
    }

    #[test]
    fn test_auth_header_without_token() {
        let client = GitHubClient::new("https://api.github.com", None).unwrap();

        let header = client.auth_header();

        assert!(header.is_none());
    }

    #[test]
    fn test_auth_header_with_token() {
        let token = Token::from("ghp_test");
        let client = GitHubClient::new("https://api.github.com", Some(token)).unwrap();

        let header = client.auth_header();

        assert!(header.is_some());
        assert_eq!(header.unwrap(), "Bearer ghp_test");
    }
}
