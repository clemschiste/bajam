#[derive(Debug)]
pub enum CliError {
    Io(std::io::Error),
    Toml(toml::de::Error),
    TomlSer(toml::ser::Error),
    Reqwest(reqwest::Error),
    SerdeJson(serde_json::Error),
    MissingConfig(String),
    UncompleteConfig,
    HttpError(reqwest::StatusCode, String),
    Auth(String),
}

impl std::fmt::Display for CliError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CliError::Io(e) => write!(f, "Erreur d'entrée/sortie: {e}"),
            CliError::Toml(e) => write!(
                f,
                "Erreur de lecture du fichier de configuration (TOML): {e}"
            ),
            CliError::TomlSer(e) => write!(
                f,
                "Erreur d'écriture du fichier de configuration (TOML): {e}"
            ),
            CliError::Reqwest(e) => write!(f, "Erreur réseau: {e}"),
            CliError::SerdeJson(e) => write!(f, "Erreur de désérialisation JSON: {e}"),
            CliError::MissingConfig(e) => write!(
                f,
                "Server config not definied. Try 'server' command first. {e}",
            ),
            CliError::UncompleteConfig => write!(f, "You must be logged in first."),
            CliError::HttpError(status, body) => write!(f, "Erreur HTTP {status}: {body}"),
            CliError::Auth(msg) => write!(f, "Erreur d'authentification: {msg}"),
        }
    }
}

impl std::error::Error for CliError {}

impl From<std::io::Error> for CliError {
    fn from(e: std::io::Error) -> Self {
        CliError::Io(e)
    }
}

impl From<toml::de::Error> for CliError {
    fn from(e: toml::de::Error) -> Self {
        CliError::Toml(e)
    }
}

impl From<toml::ser::Error> for CliError {
    fn from(e: toml::ser::Error) -> Self {
        CliError::TomlSer(e)
    }
}

impl From<reqwest::Error> for CliError {
    fn from(e: reqwest::Error) -> Self {
        CliError::Reqwest(e)
    }
}

impl From<serde_json::Error> for CliError {
    fn from(e: serde_json::Error) -> Self {
        CliError::SerdeJson(e)
    }
}
