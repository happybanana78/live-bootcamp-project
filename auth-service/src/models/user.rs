use crate::models::error::AuthAPIError;
use crate::routes::signup::SignupRequest;

#[derive(Debug, Clone)]
pub struct User {
    pub email: String,
    pub password: String,
    pub requires_2fa: bool,
}

impl User {
    pub fn new(email: String, password: String, requires_2fa: bool) -> Self {
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
        if value.email.is_empty()
            || value.password.is_empty()
            || !value.email.contains('@')
            || value.password.len() < 8
        {
            return Err(AuthAPIError::InvalidCredentials);
        }

        Ok(Self {
            email: value.email,
            password: value.password,
            requires_2fa: false,
        })
    }
}
