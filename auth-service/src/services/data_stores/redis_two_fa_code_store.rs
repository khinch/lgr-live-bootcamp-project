use std::sync::Arc;

use redis::{Commands, Connection};
use serde::{Deserialize, Serialize};
use serde_json;
use tokio::sync::RwLock;

use crate::domain::{
    Email, LoginAttemptId, TwoFACode, TwoFACodeStore, TwoFACodeStoreError,
};

pub struct RedisTwoFACodeStore {
    conn: Arc<RwLock<Connection>>,
}

impl RedisTwoFACodeStore {
    pub fn new(conn: Arc<RwLock<Connection>>) -> Self {
        Self { conn }
    }
}

#[async_trait::async_trait]
impl TwoFACodeStore for RedisTwoFACodeStore {
    async fn add_code(
        &mut self,
        email: Email,
        login_attempt_id: LoginAttemptId,
        code: TwoFACode,
    ) -> Result<(), TwoFACodeStoreError> {
        let key = get_key(&email);

        let two_fa_details = TwoFATuple(
            login_attempt_id.as_ref().to_owned(),
            code.as_ref().to_owned(),
        );

        let two_fa_details = serde_json::to_string(&two_fa_details)
            .map_err(|_| TwoFACodeStoreError::UnexpectedError)?;

        self.conn
            .write()
            .await
            .set_ex::<_, _, ()>(key, two_fa_details, TEN_MINUTES_IN_SECONDS)
            .map_err(|_| TwoFACodeStoreError::UnexpectedError)?;
        Ok(())
    }

    async fn remove_code(
        &mut self,
        email: &Email,
    ) -> Result<(), TwoFACodeStoreError> {
        let key = get_key(&email);

        self.conn
            .write()
            .await
            .del::<_, ()>(key)
            .map_err(|_| TwoFACodeStoreError::UnexpectedError)?;
        Ok(())
    }

    async fn get_code(
        &self,
        email: &Email,
    ) -> Result<(LoginAttemptId, TwoFACode), TwoFACodeStoreError> {
        let key = get_key(&email);

        let two_fa_details = self
            .conn
            .write()
            .await
            .get::<_, String>(key)
            .map_err(|_| TwoFACodeStoreError::LoginAttemptIdNotFound)?;

        let two_fa_details =
            serde_json::from_str::<TwoFATuple>(&two_fa_details)
                .map_err(|_| TwoFACodeStoreError::UnexpectedError)?;

        let (login_attempt_id, code) = (
            LoginAttemptId::parse(two_fa_details.0)
                .map_err(|_| TwoFACodeStoreError::UnexpectedError)?,
            TwoFACode::parse(two_fa_details.1)
                .map_err(|_| TwoFACodeStoreError::UnexpectedError)?,
        );

        Ok((login_attempt_id, code))
    }
}

#[derive(Serialize, Deserialize)]
struct TwoFATuple(pub String, pub String);

const TEN_MINUTES_IN_SECONDS: u64 = 600;
const TWO_FA_CODE_PREFIX: &str = "two_fa_code:";

fn get_key(email: &Email) -> String {
    format!("{}{}", TWO_FA_CODE_PREFIX, email.as_ref())
}
