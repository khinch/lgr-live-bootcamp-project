use std::collections::HashMap;

use crate::domain::User;

#[derive(Debug, PartialEq)]
pub enum UserStoreError {
    UserAlreadyExists,
    UserNotFound,
    InvalidCredentials,
    UnexpectedError,
}

#[derive(Default)]
pub struct HashmapUserStore {
    users: HashMap<String, User>,
}

impl HashmapUserStore {
    pub fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        let email = &user.email;

        if self.users.contains_key(email) {
            return Err(UserStoreError::UserAlreadyExists);
        }

        self.users.insert(email.to_string(), user);
        Ok(())
    }

    pub fn get_user(&self, email: &str) -> Result<User, UserStoreError> {
        match self.users.get(email) {
            Some(user) => Ok(User::new(
                user.email.clone(),
                user.password.clone(),
                user.requires_2fa,
            )),
            None => Err(UserStoreError::UserNotFound),
        }
    }

    pub fn validate_user(&self, email: &str, password: &str) -> Result<(), UserStoreError> {
        let user = self.get_user(email)?;
        if password.eq(&user.password) {
            Ok(())
        } else {
            Err(UserStoreError::InvalidCredentials)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_users() -> Vec<User> {
        vec![
            User::new(
                String::from("test@example.com"),
                String::from("P@55w0rd"),
                true,
            ),
            User::new(String::from("foo@bar.com"), String::from("ABC123"), false),
        ]
    }

    #[tokio::test]
    async fn test_add_user() {
        let mut users = HashmapUserStore::default();

        for test_user in get_test_users() {
            assert_eq!(
                users.add_user(test_user.clone()),
                Ok(()),
                "Failed to add user: {:?}",
                &test_user
            );
            assert_eq!(
                users.add_user(test_user.clone()),
                Err(UserStoreError::UserAlreadyExists),
                "Should not be able to add user with duplicate email"
            );
        }
    }

    #[tokio::test]
    async fn test_get_user() {
        let mut users = HashmapUserStore::default();

        for test_user in get_test_users() {
            users.add_user(test_user.clone()).unwrap();

            assert_eq!(
                users.get_user(&test_user.email),
                Ok(test_user.clone()),
                "Failed to get user with email: {:?}",
                &test_user.email
            );
        }

        assert_eq!(
            users.get_user("no@email.com"),
            Err(UserStoreError::UserNotFound),
            "User should not exist"
        );
    }

    #[tokio::test]
    async fn test_validate_user() {
        let mut users = HashmapUserStore::default();
        let valid_email = "foo@bar.com";
        let invalid_email = "lorem@ipsum.com";
        let valid_password = "P@55w0rd";
        let invalid_password = "P155w0rd";

        users
            .add_user(User::new(
                String::from(valid_email),
                String::from(valid_password),
                true,
            ))
            .unwrap();

        assert_eq!(
            users.validate_user(valid_email, valid_password),
            Ok(()),
            "User should exist with a valid password"
        );

        assert_eq!(
            users.validate_user(invalid_email, valid_password),
            Err(UserStoreError::UserNotFound),
            "User should not exist"
        );

        assert_eq!(
            users.validate_user(valid_email, invalid_password),
            Err(UserStoreError::InvalidCredentials),
            "User credentials should be invalid"
        );
    }
}
