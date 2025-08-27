use color_eyre::eyre::{Result, WrapErr};
use validator::ValidationError;

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
pub struct Email(String);

impl Email {
    pub fn parse(email: String) -> Result<Self> {
        if !validator::validate_email(&email) {
            let mut error = ValidationError::new("Invalid email address");
            error.message = Some("For more details, see the spec: https://html.spec.whatwg.org/multipage/input.html#valid-e-mail-address".into());
            return Err(error).wrap_err("failed to parse email");
        }

        Ok(Email(email))
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
            let parsed =
                Email::parse(valid_email.to_string()).expect(valid_email);
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
            let result = Email::parse(invalid_email.to_string());
            let error = result.expect_err(invalid_email);

            // Downcast to get the original ValidationError
            let validation_error = error
                .downcast_ref::<ValidationError>()
                .expect("Expected ValidationError");

            assert_eq!(validation_error.code, "Invalid email address");
            assert_eq!(validation_error.message.as_ref().unwrap(), "For more details, see the spec: https://html.spec.whatwg.org/multipage/input.html#valid-e-mail-address");
        }
    }
}
