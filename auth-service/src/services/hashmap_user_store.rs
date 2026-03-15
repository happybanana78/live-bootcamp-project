use crate::models::data_store::{UserStore, UserStoreError};
use crate::models::email::Email;
use crate::models::password::Password;
use crate::models::user::User;
use std::collections::hash_map::Entry;
use std::collections::HashMap;

#[derive(Default)]
pub struct HashmapUserStore {
    users: HashMap<Email, User>,
}

impl UserStore for HashmapUserStore {
    fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        match self.users.entry(user.email.clone()) {
            Entry::Occupied(_) => Err(UserStoreError::UserAlreadyExists),
            Entry::Vacant(v) => {
                v.insert(user);
                Ok(())
            }
        }
    }

    fn get_user(&self, email: Email) -> Result<User, UserStoreError> {
        match self.users.get(&email) {
            Some(user) => Ok(user.clone()),
            None => Err(UserStoreError::UserNotFound),
        }
    }

    fn validate_user(&self, email: Email, password: Password) -> Result<(), UserStoreError> {
        let user = self.get_user(email)?;

        if user.password.as_ref() == password.as_ref() {
            Ok(())
        } else {
            Err(UserStoreError::InvalidCredentials)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_add_user() {
        let user = User::new(
            Email::parse("user1@example.com").unwrap(),
            Password::parse("password123").unwrap(),
            false,
        );

        let mut store = HashmapUserStore::default();

        let result = store.add_user(user);

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_user() {
        let email = Email::parse("user1@example.com").unwrap();

        let user = User::new(
            email.clone(),
            Password::parse("password123").unwrap(),
            false,
        );

        let mut store = HashmapUserStore::default();
        store.add_user(user).unwrap();

        let user_from_store = store.get_user(email.clone()).unwrap();

        assert_eq!(user_from_store.email, email);
    }

    #[tokio::test]
    async fn test_validate_user() {
        let email = Email::parse("user1@example.com").unwrap();
        let password = Password::parse("password123").unwrap();

        let user = User::new(email.clone(), password.clone(), false);

        let mut store = HashmapUserStore::default();
        store.add_user(user).unwrap();

        let result = store.validate_user(email, password);

        assert!(result.is_ok());
    }
}
