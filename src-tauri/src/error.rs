use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};

/// Erreur renvoyée à l'interface : un `code` stable qu'elle traduit, et un message pour le détail.
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("réseau indisponible : {0}")]
    Network(String),
    #[error("limite de l'API GitHub atteinte")]
    RateLimited { reset: Option<i64> },
    #[error("réponse inattendue de GitHub (HTTP {0})")]
    GitHub(u16),
    #[error("jeu inconnu : {0}")]
    UnknownGame(String),
    #[error("catalogue invalide : {0}")]
    InvalidCatalog(String),
    #[error("fichier trop gros : {0}")]
    TooLarge(String),
    #[error("erreur de fichier : {0}")]
    Io(String),
}

impl AppError {
    fn code(&self) -> &'static str {
        match self {
            Self::Network(_) => "network",
            Self::RateLimited { .. } => "rateLimited",
            Self::GitHub(_) => "github",
            Self::UnknownGame(_) => "unknownGame",
            Self::InvalidCatalog(_) => "invalidCatalog",
            Self::TooLarge(_) => "tooLarge",
            Self::Io(_) => "io",
        }
    }
}

impl From<reqwest::Error> for AppError {
    fn from(error: reqwest::Error) -> Self {
        Self::Network(error.to_string())
    }
}

impl From<std::io::Error> for AppError {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error.to_string())
    }
}

impl Serialize for AppError {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut state = serializer.serialize_struct("AppError", 3)?;
        state.serialize_field("code", self.code())?;
        state.serialize_field("message", &self.to_string())?;
        let reset = match self {
            Self::RateLimited { reset } => *reset,
            _ => None,
        };
        state.serialize_field("reset", &reset)?;
        state.end()
    }
}
