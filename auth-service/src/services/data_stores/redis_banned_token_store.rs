use std::sync::Arc;

use redis::{Commands, Connection};
use tokio::sync::RwLock;

use crate::{
    domain::{BannedTokenStore, TokenStoreError},
    utils::auth::TOKEN_TTL_SECONDS,
};

pub struct RedisBannedTokenStore {
    conn: Arc<RwLock<Connection>>,
}

impl RedisBannedTokenStore {
    pub fn new(conn: Arc<RwLock<Connection>>) -> Self {
        Self { conn }
    }
}

#[async_trait::async_trait]
impl BannedTokenStore for RedisBannedTokenStore {
    async fn add_token(&mut self, token: &str) -> Result<(), TokenStoreError> {
        let key = get_key(token);
        let token_ttl_seconds: u64 = TOKEN_TTL_SECONDS
            .try_into()
            .map_err(|_| TokenStoreError::UnexpectedError)?;

        self.conn
            .write()
            .await
            .set_ex::<_, _, ()>(key, true, token_ttl_seconds)
            .map_err(|_| TokenStoreError::UnexpectedError)?;

        Ok(())
    }

    async fn check_token(&self, token: &str) -> Result<(), TokenStoreError> {
        let key = get_key(&token);
        match self.conn.write().await.exists(&key) {
            Ok(true) => Err(TokenStoreError::BannedToken),
            Ok(false) => Ok(()),
            Err(_) => Err(TokenStoreError::UnexpectedError),
        }
    }
}

// We are using a key prefix to prevent collisions and organize data!
const BANNED_TOKEN_KEY_PREFIX: &str = "banned_token:";

fn get_key(token: &str) -> String {
    format!("{}{}", BANNED_TOKEN_KEY_PREFIX, token)
}
