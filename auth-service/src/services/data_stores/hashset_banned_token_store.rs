use color_eyre::eyre::Result;
use std::collections::HashSet;

use crate::domain::{BannedTokenStore, BannedTokenStoreError};

#[derive(Default)]
pub struct HashsetBannedTokenStore {
    banned_tokens: HashSet<String>,
}

#[async_trait::async_trait]
impl BannedTokenStore for HashsetBannedTokenStore {
    async fn add_token(&mut self, token: &str) -> Result<()> {
        self.banned_tokens.insert(String::from(token));
        Ok(())
    }

    async fn check_token(
        &self,
        token: &str,
    ) -> Result<(), BannedTokenStoreError> {
        if self.banned_tokens.contains(token) {
            Err(BannedTokenStoreError::BannedToken)
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_add_token() {
        let mut banned_tokens = HashsetBannedTokenStore::default();
        let token = "token";

        assert!(
            banned_tokens.add_token(&token).await.is_ok(),
            "Failed to add token to store"
        );
        assert!(
            banned_tokens.add_token(&token).await.is_ok(),
            "Failed to add token to store"
        );
    }

    #[tokio::test]
    async fn test_get_user() {
        let mut banned_tokens = HashsetBannedTokenStore::default();
        let token = "token";

        assert!(
            banned_tokens.check_token(&token).await.is_ok(),
            "Token banned without existing in store"
        );
        assert!(
            banned_tokens.add_token(&token).await.is_ok(),
            "Failed to add token to store"
        );
        assert_eq!(
            banned_tokens.check_token(&token).await,
            Err(BannedTokenStoreError::BannedToken),
            "Token should be banned"
        );
    }
}
