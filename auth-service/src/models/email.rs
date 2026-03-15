use validator::ValidationError;

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct Email(String);

impl Email {
    pub fn parse(email: &str) -> Result<Email, ValidationError> {
        if !email.contains("@") {
            return Err(ValidationError::new("Email must contain '@' symbol"));
        }

        Ok(Email(email.to_string()))
    }
}

impl AsRef<str> for Email {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_email_parse_valid() {
        let email = Email::parse("test@example.com");
        assert!(email.is_ok());
    }

    #[test]
    fn test_email_parse_invalid() {
        let email = Email::parse("invalid_email");
        assert!(email.is_err());
    }
}
