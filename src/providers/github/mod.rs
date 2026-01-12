mod cache;
pub mod client;
mod provider;
mod types;

pub use cache::JobCache;
pub use provider::GitHubProvider;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_github_provider_exists() {
        let _: Option<GitHubProvider> = None;
    }
}
