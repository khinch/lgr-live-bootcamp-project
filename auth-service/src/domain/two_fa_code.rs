#[derive(Clone, Debug, PartialEq, serde::Serialize)]
pub struct TwoFACode(String);

impl TwoFACode {
    pub fn parse(code: String) -> Result<Self, String> {
        let regex = regex::Regex::new(r"^\d{6}$").unwrap();
        if regex.is_match(&code) {
            Ok(TwoFACode(code))
        } else {
            Err(String::from("Code is invalid"))
        }
    }
}

impl Default for TwoFACode {
    fn default() -> Self {
        let code = rand::random::<u32>() % 1_000_000;
        TwoFACode(format!("{:06}", code))
    }
}

impl AsRef<str> for TwoFACode {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_codes() {
        let valid_codes = ["123456", "654321", "000000", "999999"];
        for valid_code in valid_codes.iter() {
            let parsed = TwoFACode::parse(valid_code.to_string()).expect(valid_code);
            assert_eq!(
                &parsed.as_ref(),
                valid_code,
                "Code does not match expected value"
            );
        }
    }

    #[test]
    fn test_invalid_codes() {
        let invalid_codes = ["12345", "1234567", "12345a", "a12345"];
        for invalid_code in invalid_codes.iter() {
            let result = TwoFACode::parse(invalid_code.to_string());
            let error = result.expect_err(invalid_code);
            assert_eq!(error, "Code is invalid");
        }
    }
}
