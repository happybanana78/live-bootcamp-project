use crate::models::email::Email;
use crate::models::error::AuthAPIError;
use crate::models::password::Password;
use crate::routes::signup::SignupRequest;

#[derive(Debug, Clone)]
pub struct User {
    pub email: Email,
    pub password: Password,
    pub requires_2fa: bool,
}

impl User {
    pub fn new(email: Email, password: Password, requires_2fa: bool) -> Self {
        Self {
            email,
            password,
            requires_2fa,
        }
    }
}

impl TryFrom<SignupRequest> for User {
    type Error = AuthAPIError;

    fn try_from(value: SignupRequest) -> Result<Self, Self::Error> {
        let email = Email::parse(&value.email).map_err(|_| AuthAPIError::InvalidCredentials)?;

        let password =
            Password::parse(&value.password).map_err(|_| AuthAPIError::InvalidCredentials)?;

        Ok(Self {
            email,
            password,
            requires_2fa: false,
        })
    }
}
