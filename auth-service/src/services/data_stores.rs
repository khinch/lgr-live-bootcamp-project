use std::collections::{HashMap, HashSet};

use crate::domain::{
    BannedTokenStore, Email, LoginAttemptId, Password, TokenStoreError,
    TwoFACode, TwoFACodeStore, TwoFACodeStoreError, User, UserStore,
    UserStoreError,
};

#[derive(Default)]
pub struct HashmapTwoFACodeStore {
    codes: HashMap<Email, (LoginAttemptId, TwoFACode)>,
}

#[async_trait::async_trait]
impl TwoFACodeStore for HashmapTwoFACodeStore {
    async fn add_code(
        &mut self,
        email: Email,
        login_attempt_id: LoginAttemptId,
        code: TwoFACode,
    ) -> Result<(), TwoFACodeStoreError> {
        self.codes.insert(email, (login_attempt_id, code));
        Ok(())
    }

    async fn remove_code(
        &mut self,
        email: &Email,
    ) -> Result<(), TwoFACodeStoreError> {
        self.codes.remove(email);
        Ok(())
    }

    async fn get_code(
        &self,
        email: &Email,
    ) -> Result<(LoginAttemptId, TwoFACode), TwoFACodeStoreError> {
        match self.codes.get(email) {
            Some((id, code)) => Ok((id.clone(), code.clone())),
            None => Err(TwoFACodeStoreError::LoginAttemptIdNotFound),
        }
    }
}

#[derive(Default)]
pub struct HashmapUserStore {
    users: HashMap<Email, User>,
}

#[async_trait::async_trait]
impl UserStore for HashmapUserStore {
    async fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        let email = &user.email;

        if self.users.contains_key(email) {
            return Err(UserStoreError::UserAlreadyExists);
        }

        self.users.insert(email.clone(), user);
        Ok(())
    }

    async fn get_user(&self, email: Email) -> Result<User, UserStoreError> {
        match self.users.get(&email) {
            Some(user) => Ok(user.clone()),
            None => Err(UserStoreError::UserNotFound),
        }
    }

    async fn validate_user(
        &self,
        email: &Email,
        password: &Password,
    ) -> Result<(), UserStoreError> {
        let user = self.get_user(email.clone()).await?;
        if password.eq(&user.password) {
            Ok(())
        } else {
            Err(UserStoreError::InvalidCredentials)
        }
    }

    async fn delete_user(
        &mut self,
        email: &Email,
    ) -> Result<(), UserStoreError> {
        match self.users.remove(email) {
            Some(_) => Ok(()),
            None => Err(UserStoreError::UserNotFound),
        }
    }
}

#[derive(Default)]
pub struct HashsetBannedTokenStore {
    banned_tokens: HashSet<String>,
}

#[async_trait::async_trait]
impl BannedTokenStore for HashsetBannedTokenStore {
    async fn add_token(&mut self, token: &str) -> Result<(), TokenStoreError> {
        self.banned_tokens.insert(String::from(token));
        Ok(())
    }

    async fn check_token(&self, token: &str) -> Result<(), TokenStoreError> {
        if self.banned_tokens.contains(token) {
            Err(TokenStoreError::BannedToken)
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {

    mod hashmap_two_fa_code_store {
        use super::super::*;

        fn get_test_data() -> (Email, LoginAttemptId, TwoFACode) {
            let email = Email::parse(String::from("foo@bar.com"))
                .expect("Could not parse email");
            let id = LoginAttemptId::parse(String::from(
                "b65b6b5a-cae7-436b-8196-16abcfb59e47",
            ))
            .expect("Could not parse LoginAttemptId");
            let code = TwoFACode::parse(String::from("123456"))
                .expect("Could not parse 2FA code");
            (email, id, code)
        }

        #[tokio::test]
        async fn add_code() {
            let (email, id, code) = get_test_data();
            let mut store = HashmapTwoFACodeStore::default();
            assert_eq!(
                store
                    .add_code(email.clone(), id.clone(), code.clone())
                    .await,
                Ok(()),
                "Failed to add 2FA data to store"
            );
            assert!(store.codes.contains_key(&email));
        }

        #[tokio::test]
        async fn get_code() {
            let (email, id, code) = get_test_data();
            let mut store = HashmapTwoFACodeStore::default();
            assert_eq!(
                store
                    .add_code(email.clone(), id.clone(), code.clone())
                    .await,
                Ok(()),
                "Failed to add 2FA data to store"
            );
            assert_eq!(
                store.get_code(&email).await.unwrap(),
                (id, code),
                "Retrieved code does not match stored code"
            );
        }

        #[tokio::test]
        async fn remove_code() {
            let (email, id, code) = get_test_data();
            let mut store = HashmapTwoFACodeStore::default();
            assert_eq!(
                store
                    .add_code(email.clone(), id.clone(), code.clone())
                    .await,
                Ok(()),
                "Failed to add 2FA data to store"
            );
            assert!(store.codes.contains_key(&email));
            assert_eq!(
                store.remove_code(&email).await,
                Ok(()),
                "Failed to remove code"
            );
            assert!(!store.codes.contains_key(&email));
        }

        #[tokio::test]
        async fn get_non_existent_code_returns_error() {
            let (email, _id, _code) = get_test_data();
            let store = HashmapTwoFACodeStore::default();
            assert_eq!(
                store.get_code(&email).await,
                Err(TwoFACodeStoreError::LoginAttemptIdNotFound),
                "Non-existent code should return error"
            );
        }

        #[tokio::test]
        async fn code_can_be_updated() {
            let (email, id, code) = get_test_data();
            let mut store = HashmapTwoFACodeStore::default();
            assert_eq!(
                store
                    .add_code(email.clone(), id.clone(), code.clone())
                    .await,
                Ok(()),
                "Failed to add 2FA data to store"
            );

            let updated_id = LoginAttemptId::parse(String::from(
                "3a6fe309-45a9-49a6-ad44-4a5411760ae3",
            ))
            .expect("Could not parse LoginAttemptId");
            let updated_code = TwoFACode::parse(String::from("654321"))
                .expect("Could not parse 2FA code");

            assert_eq!(
                store
                    .add_code(
                        email.clone(),
                        updated_id.clone(),
                        updated_code.clone()
                    )
                    .await,
                Ok(()),
                "Failed to update 2FA data in store"
            );

            assert_eq!(
                store.get_code(&email).await.unwrap(),
                (updated_id, updated_code),
                "Retrieved code does not match updated code"
            );
        }

        #[tokio::test]
        async fn removing_code_multiple_times_is_idempotent() {
            let (email, id, code) = get_test_data();
            let mut store = HashmapTwoFACodeStore::default();
            assert_eq!(
                store
                    .add_code(email.clone(), id.clone(), code.clone())
                    .await,
                Ok(()),
                "Failed to add 2FA data to store"
            );
            assert!(store.codes.contains_key(&email));
            assert_eq!(
                store.remove_code(&email).await,
                Ok(()),
                "Failed to remove code"
            );
            assert!(!store.codes.contains_key(&email));

            assert_eq!(
                store.remove_code(&email).await,
                Ok(()),
                "Failed attempt to remove non-existent code"
            );
            assert!(!store.codes.contains_key(&email));
        }
    }

    mod hashmap_user_store {
        use super::super::*;

        fn get_test_users() -> Vec<User> {
            vec![
                User::new(
                    Email::parse("test@example.com".to_string()).unwrap(),
                    Password::parse("P@55w0rd".to_string()).unwrap(),
                    true,
                ),
                User::new(
                    Email::parse("foo@bar.com".to_string()).unwrap(),
                    Password::parse("ABCD1234".to_string()).unwrap(),
                    false,
                ),
            ]
        }

        #[tokio::test]
        async fn test_add_user() {
            let mut users = HashmapUserStore::default();

            for test_user in get_test_users() {
                assert_eq!(
                    users.add_user(test_user.clone()).await,
                    Ok(()),
                    "Failed to add user: {:?}",
                    &test_user
                );
                assert_eq!(
                    users.add_user(test_user.clone()).await,
                    Err(UserStoreError::UserAlreadyExists),
                    "Should not be able to add user with duplicate email"
                );
            }
        }

        #[tokio::test]
        async fn test_get_user() {
            let mut users = HashmapUserStore::default();

            for test_user in get_test_users() {
                users.add_user(test_user.clone()).await.unwrap();

                assert_eq!(
                    users.get_user(test_user.email.clone()).await,
                    Ok(test_user.clone()),
                    "Failed to get user with email: {:?}",
                    &test_user.email
                );
            }

            let non_existent_user =
                Email::parse("no@email.com".to_string()).unwrap();
            assert_eq!(
                users.get_user(non_existent_user).await,
                Err(UserStoreError::UserNotFound),
                "User should not exist"
            );
        }

        #[tokio::test]
        async fn test_validate_user() {
            let mut users = HashmapUserStore::default();
            let existent_email =
                Email::parse("foo@bar.com".to_string()).unwrap();
            let non_existent_email =
                Email::parse("lorem@ipsum.com".to_string()).unwrap();
            let existent_password =
                Password::parse("P@55w0rd".to_string()).unwrap();
            let non_existent_password =
                Password::parse("P155w0rd".to_string()).unwrap();

            users
                .add_user(User::new(
                    existent_email.clone(),
                    existent_password.clone(),
                    true,
                ))
                .await
                .unwrap();

            assert_eq!(
                users
                    .validate_user(&existent_email, &existent_password)
                    .await,
                Ok(()),
                "User should exist with a valid password"
            );

            assert_eq!(
                users
                    .validate_user(&non_existent_email, &existent_password)
                    .await,
                Err(UserStoreError::UserNotFound),
                "User should not exist"
            );

            assert_eq!(
                users
                    .validate_user(&existent_email, &non_existent_password)
                    .await,
                Err(UserStoreError::InvalidCredentials),
                "User credentials should be invalid"
            );
        }

        #[tokio::test]
        async fn test_delete_user() {
            let mut users = HashmapUserStore::default();

            let user = User::new(
                Email::parse("test@example.com".to_string()).unwrap(),
                Password::parse("P@55w0rd".to_string()).unwrap(),
                true,
            );

            // Should be able to re-add and re-delete
            for _ in 0..2 {
                users
                    .add_user(user.clone())
                    .await
                    .expect(user.email.as_ref());

                assert_eq!(
                    users.delete_user(&user.email).await,
                    Ok(()),
                    "Failed to delete user"
                );
                assert_eq!(
                    users.delete_user(&user.email).await,
                    Err(UserStoreError::UserNotFound),
                    "User should not have existed"
                );
            }
        }
    }

    mod hashset_banned_token_store {
        use super::super::*;

        #[tokio::test]
        async fn test_add_token() {
            let mut banned_tokens = HashsetBannedTokenStore::default();
            let token = "token";

            assert_eq!(
                banned_tokens.add_token(&token).await,
                Ok(()),
                "Failed to add token to store"
            );
            assert_eq!(
                banned_tokens.add_token(&token).await,
                Ok(()),
                "Failed to add token to store"
            );
        }

        #[tokio::test]
        async fn test_get_user() {
            let mut banned_tokens = HashsetBannedTokenStore::default();
            let token = "token";

            assert_eq!(
                banned_tokens.check_token(&token).await,
                Ok(()),
                "Token banned without existing in store"
            );
            assert_eq!(
                banned_tokens.add_token(&token).await,
                Ok(()),
                "Failed to add token to store"
            );
            assert_eq!(
                banned_tokens.check_token(&token).await,
                Err(TokenStoreError::BannedToken),
                "Token should be banned"
            );
        }
    }
}
