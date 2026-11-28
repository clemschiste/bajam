use serde::{Deserialize, Serialize};
// SQL struct
#[derive(Debug, Serialize, Deserialize)]
pub struct Escapade {
    pub id: i64,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EscapadeResponse {
    pub escapade_id: i64,
}

#[derive(Debug, PartialEq, thiserror::Error)]
pub enum EscapadePostError {
    #[error("name must not be empty")]
    EmptyName,
}

/// Parsed escapade. Can only be built through `new`, or by deserializing
/// (which goes through `new`), so a `EscapadePost` is always valid.
#[derive(Debug, Deserialize)]
#[serde(try_from = "RawEscapadePost")]
pub struct EscapadePost {
    pub name: String,
}

#[derive(Deserialize)]
struct RawEscapadePost {
    name: String,
}

impl TryFrom<RawEscapadePost> for EscapadePost {
    type Error = EscapadePostError;

    fn try_from(raw: RawEscapadePost) -> Result<Self, Self::Error> {
        Self::new(&raw.name)
    }
}

impl EscapadePost {
    pub fn new(name: &str) -> Result<Self, EscapadePostError> {
        let name = name.trim();
        if name.is_empty() {
            return Err(EscapadePostError::EmptyName);
        }
        Ok(Self { name: name.into() })
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_escapade() {
        let e = EscapadePost::new("japan - 2026").unwrap();

        assert_eq!(e.name, "japan - 2026".to_string());
    }

    #[test]
    fn escapade_empty_name_rejected() {
        let e = EscapadePost::new("");

        assert_eq!(e.unwrap_err(), EscapadePostError::EmptyName);
    }
}
