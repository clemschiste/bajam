use serde::Deserialize;

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
