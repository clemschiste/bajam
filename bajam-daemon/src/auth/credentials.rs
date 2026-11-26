use serde::Deserialize;

#[derive(Debug, PartialEq, thiserror::Error)]
pub enum CredentialsError {
    #[error("username must not be empty")]
    EmptyUsername,
    #[error("password must not be empty")]
    EmptyPassword,
}

/// Parsed username/password. Can only be built through `new`, or by deserializing
/// (which goes through `new`), so a `Credentials` is always valid.
#[derive(Debug, Deserialize)]
#[serde(try_from = "RawCredentials")]
pub struct Credentials {
    username: String,
    password: String,
}

#[derive(Deserialize)]
struct RawCredentials {
    username: String,
    password: String,
}

impl TryFrom<RawCredentials> for Credentials {
    type Error = CredentialsError;

    fn try_from(raw: RawCredentials) -> Result<Self, Self::Error> {
        Self::new(&raw.username, &raw.password)
    }
}

impl Credentials {
    pub fn new(username: &str, password: &str) -> Result<Self, CredentialsError> {
        let username = username.trim();
        if username.is_empty() {
            return Err(CredentialsError::EmptyUsername);
        }
        if password.is_empty() {
            return Err(CredentialsError::EmptyPassword);
        }
        Ok(Self { username: username.into(), password: password.into() })
    }

    pub fn username(&self) -> &str {
        &self.username
    }

    pub fn password(&self) -> &str {
        &self.password
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_credentials_parse() {
        let c = Credentials::new("user", "secret").unwrap();

        assert_eq!((c.username(), c.password()), ("user", "secret"));
    }

    #[test]
    fn username_is_trimmed() {
        let c = Credentials::new("  user ", "secret").unwrap();

        assert_eq!(c.username(), "user");
    }

    #[test]
    fn blank_username_rejected() {
        let res = Credentials::new("   ", "secret");

        assert_eq!(res.unwrap_err(), CredentialsError::EmptyUsername);
    }

    #[test]
    fn empty_password_rejected() {
        let res = Credentials::new("user", "");

        assert_eq!(res.unwrap_err(), CredentialsError::EmptyPassword);
    }

    #[test]
    fn deserialization_validates() {
        let res: Result<Credentials, _> = serde_json::from_str(r#"{"username":"","password":"x"}"#);

        assert!(res.is_err());
    }
}
