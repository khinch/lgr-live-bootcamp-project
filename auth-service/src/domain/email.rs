use core::convert::AsRef;
use validator::ValidationError;

#[derive(Debug, PartialEq)]
pub struct Email(String);

impl Email {
    pub fn parse(email: &str) -> Result<Self, ValidationError> {
        if !validator::validate_email(email) {
            // return Err(ValidationError::new("Invalid email address. For more details, see the spec: https://html.spec.whatwg.org/multipage/input.html#valid-e-mail-address"));
            let mut error = ValidationError::new("Invalid email address");
            error.message = Some("For more details, see the spec: https://html.spec.whatwg.org/multipage/input.html#valid-e-mail-address".into());
            return Err(error);
        }

        Ok(Email(String::from(email)))
    }
}

impl AsRef<str> for Email {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /*
     * Validation is performed by the `validator` library.
     * Unit tests don't need to be comprehensive, just a sanity check
     * to ensure the library has been correctly implemented.
     * Docs:
     * https://html.spec.whatwg.org/multipage/input.html#valid-e-mail-address
     */
    #[test]
    fn test_valid_emails() {
        let valid_emails = ["a@b", "foo@bar.com"];
        for valid_email in valid_emails.iter() {
            let parsed = Email::parse(valid_email).expect(valid_email);
            assert_eq!(
                &parsed.as_ref(),
                valid_email,
                "Email does not match expected value"
            );
        }
    }

    #[test]
    fn test_invalid_emails() {
        let invalid_emails = ["ab.com", "foo.bar"];
        for invalid_email in invalid_emails.iter() {
            let result = Email::parse(invalid_email);
            let error = result.expect_err(invalid_email);
            assert_eq!(error.code, "Invalid email address");
            assert_eq!(error.message.unwrap(), "For more details, see the spec: https://html.spec.whatwg.org/multipage/input.html#valid-e-mail-address");
        }
    }
}
