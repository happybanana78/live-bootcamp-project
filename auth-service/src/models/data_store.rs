use crate::models::email::Email;
use crate::models::password::Password;
use crate::models::user::User;

#[async_trait::async_trait]
pub trait UserStore: Send + Sync {
    fn add_user(&mut self, user: User) -> Result<(), UserStoreError>;

    fn get_user(&self, email: Email) -> Result<User, UserStoreError>;

    fn validate_user(&self, email: Email, password: Password) -> Result<(), UserStoreError>;
}

#[derive(Debug, PartialEq)]
pub enum UserStoreError {
    UserAlreadyExists,
    UserNotFound,
    InvalidCredentials,
    UnexpectedError,
}
