use crate::models::User;
use std::collections::hash_map::Entry;
use std::collections::HashMap;

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
        match self.users.entry(user.email.clone()) {
            Entry::Occupied(_) => Err(UserStoreError::UserAlreadyExists),
            Entry::Vacant(v) => {
                v.insert(user);
                Ok(())
            }
        }
    }

    pub fn get_user(&self, email: &str) -> Result<User, UserStoreError> {
        match self.users.get(email) {
            Some(user) => Ok(user.clone()),
            None => Err(UserStoreError::UserNotFound),
        }
    }

    pub fn validate_user(&self, email: &str, password: &str) -> Result<(), UserStoreError> {
        let user = self.get_user(email)?;

        if user.password == password {
            Ok(())
        } else {
            Err(UserStoreError::InvalidCredentials)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::models::User;
    use crate::services::hashmap_user_store::HashmapUserStore;

    #[tokio::test]
    async fn test_add_user() {
        let user = User::new(
            "user1@example.com".to_string(),
            "password123".to_string(),
            false,
        );

        let mut store = HashmapUserStore::default();

        let result = store.add_user(user);

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_user() {
        let user = User::new(
            "user1@example.com".to_string(),
            "password123".to_string(),
            false,
        );

        let mut store = HashmapUserStore::default();
        store.add_user(user).unwrap();

        let user_from_store = store.get_user("user1@example.com").unwrap();

        assert_eq!(user_from_store.email, "user1@example.com");
    }

    #[tokio::test]
    async fn test_validate_user() {
        let user = User::new(
            "user1@example.com".to_string(),
            "password123".to_string(),
            false,
        );

        let mut store = HashmapUserStore::default();
        store.add_user(user).unwrap();

        let result = store.validate_user("user1@example.com", "password123");

        assert!(result.is_ok());
    }
}
