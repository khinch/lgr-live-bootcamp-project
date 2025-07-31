use validator::ValidationError;

#[derive(Debug)]
pub struct Password(String);

impl Password {
    pub fn parse(password: &str) -> Result<Self, ValidationError> {
        match validate_password(password) {
            Ok(()) => Ok(Password(String::from(password))),
            Err(message) => {
                let mut error = ValidationError::new("Invalid password");
                error.message = Some(message.into());
                Err(error)
            }
        }
    }
}

fn validate_password(password: &str) -> Result<(), String> {
    let min_characters = 8;
    let max_characters = 64;
    let char_count = password.chars().count();

    if char_count < min_characters {
        return Err(format!(
            "Too short. Should be {} to {} characters.",
            min_characters, max_characters
        ));
    }

    if char_count > max_characters {
        return Err(format!(
            "Too long. Should be {} to {} characters.",
            min_characters, max_characters,
        ));
    }

    Ok(())
}

impl AsRef<str> for Password {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_passwords() {
        let valid_passwords = [
            "12345678",
            "abcdefghijklmnopqrstuvwxyz1234567890ABCDEFGHIJKLMNOPQRSTUVWXYZ12",
            r##"`¬!"£$%^&*()_-=+[]{}|\'@#~;:/?<>,.\\\\\\\\\\"##,
            "😀😁😂😃😄😅😆😎",
            "☀☁☂☃☄★☆☇☈☉☊☋☌☍☎☏☐☑☒☓☔☕ħĨ☘☙☚☛☜☝☞☟☠☡☢☣ĩ☥☦☧☨☩☪☫☬☭☮☯☰☱☲☳☴☵☶☷☸☹☺☻☼☽☾☿",
        ];
        for valid_password in valid_passwords.iter() {
            let parsed = Password::parse(valid_password).expect(valid_password);
            assert_eq!(&parsed.as_ref(), valid_password);
        }
    }

    #[test]
    fn test_short_passwords() {
        let short_passwords = ["1234567", "😀😁😂😃😄😅😆"];
        for short_password in short_passwords.iter() {
            let result = Password::parse(short_password);
            let error = result.expect_err(short_password);
            assert_eq!(error.code, "Invalid password");
            assert!(error.message.unwrap().starts_with("Too short"));
        }
    }

    #[test]
    fn test_long_passwords() {
        let long_passwords = [
            "abcdefghijklmnopqrstuvwxyz1234567890ABCDEFGHIJKLMNOPQRSTUVWXYZ123",
            "☀☁☂☃☄★☆☇☈☉☊☋☌☍☎☏☐☑☒☓☔☕ħĨ☘☙☚☛☜☝☞☟☠☡☢☣ĩ☥☦☧☨☩☪☫☬☭☮☯☰☱☲☳☴☵☶☷☸☹☺☻☼☽☾☿♀",
        ];
        for long_password in long_passwords.iter() {
            let result = Password::parse(long_password);
            let error = result.expect_err(long_password);
            assert_eq!(error.code, "Invalid password");
            assert!(error.message.unwrap().starts_with("Too long"));
        }
    }
}
