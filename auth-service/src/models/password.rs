use validator::ValidationError;

#[derive(Debug, Clone)]
pub struct Password(String);

impl Password {
    pub fn parse(pwd: &str) -> Result<Password, ValidationError> {
        if pwd.len() < 8 {
            return Err(ValidationError::new(
                "Password must be at least 8 characters long",
            ));
        }

        Ok(Password(pwd.to_string()))
    }
}

impl AsRef<str> for Password {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_parse_valid() {
        let password = Password::parse("12345678");
        assert!(password.is_ok());
    }

    #[test]
    fn test_password_parse_invalid() {
        let password = Password::parse("123");
        assert!(password.is_err());
    }
}
