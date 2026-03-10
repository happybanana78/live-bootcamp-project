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

impl From<SignupRequest> for User {
    fn from(request: SignupRequest) -> Self {
        Self {
            email: request.email,
            password: request.password,
            requires_2fa: false,
        }
    }
}
