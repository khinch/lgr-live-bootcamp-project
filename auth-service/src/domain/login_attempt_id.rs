use color_eyre::eyre::{Context, Result};

#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct LoginAttemptId(String);

impl LoginAttemptId {
    pub fn parse(id: String) -> Result<Self> {
        let parsed =
            uuid::Uuid::try_parse(&id).wrap_err("Invalid login attempt ID")?;
        Ok(Self(parsed.to_string()))
    }
}

impl Default for LoginAttemptId {
    fn default() -> Self {
        let id = String::from(uuid::Uuid::new_v4());
        LoginAttemptId(id)
    }
}

impl AsRef<str> for LoginAttemptId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_ids() {
        let valid_ids = [
            "5e90ca28-e1ad-4795-a190-089959c16e0b",
            "5b5b32e3-66cc-45bc-82d1-d41582139f1e",
            "00000000-0000-0000-0000-000000000000",
            "99999999-9999-9999-9999-999999999999",
            "aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaaa",
            "ffffffff-ffff-ffff-ffff-ffffffffffff",
        ];
        for valid_id in valid_ids.iter() {
            let parsed =
                LoginAttemptId::parse(valid_id.to_string()).expect(valid_id);
            assert_eq!(
                &parsed.as_ref(),
                valid_id,
                "ID does not match expected value"
            );
        }
    }

    #[test]
    fn test_invalid_ids() {
        let invalid_ids = [
            "5b5b32e3a66cc-45bc-82d1-d41582139f1e",
            "5b5b32e3-66cca45bc-82d1-d41582139f1e",
            "5b5b32e3-66cc-45bca82d1-d41582139f1e",
            "5b5b32e3-66cc-45bc-82d1ad41582139f1e",
            "b5b32e3-66cc-45bc-82d1-d41582139f1e",
            "5b5b32e3-6cc-45bc-82d1-d41582139f1e",
            "5b5b32e3-66cc-5bc-82d1-d41582139f1e",
            "5b5b32e3-66cc-45bc-2d1-d41582139f1e",
            "5b5b32e3-66cc-45bc-82d1-41582139f1e",
            "5b5b32e3a-66cc-45bc-82d1-d41582139f1e",
            "5b5b32e3-66cca-45bc-82d1-d41582139f1e",
            "5b5b32e3-66cc-45bca-82d1-d41582139f1e",
            "5b5b32e3-66cc-45bc-82d1a-d41582139f1e",
            "5b5b32e3-66cc-45bc-82d1-d41582139f1ea",
        ];
        for invalid_id in invalid_ids.iter() {
            let result = LoginAttemptId::parse(invalid_id.to_string());
            let error = result.expect_err(invalid_id);
            assert_eq!(error.to_string(), "Invalid login attempt ID");
        }
    }
}
