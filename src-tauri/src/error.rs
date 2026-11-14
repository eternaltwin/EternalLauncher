use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};
use serde_json::{Value, json};

/// Erreur renvoyée à l'interface : un `code` stable qu'elle traduit, un message pour le détail,
/// et parfois des données (`data`) pour compléter le texte.
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
    #[error("version introuvable : {0}")]
    UnknownVersion(String),
    #[error("catalogue invalide : {0}")]
    InvalidCatalog(String),
    #[error("fichier trop gros : {0}")]
    TooLarge(String),
    #[error("le jeu n'existe pas pour ce système")]
    Unavailable,
    #[error("GitHub ne donne pas l'empreinte de ce fichier")]
    MissingDigest,
    #[error("le fichier téléchargé ne correspond pas à son empreinte")]
    DigestMismatch,
    #[error("archive refusée : {0}")]
    BadArchive(String),
    #[error("téléchargement annulé")]
    Cancelled,
    #[error("cette version est déjà en cours d'installation")]
    Busy,
    #[error("cette version n'est pas installée")]
    NotInstalled,
    #[error("{runtime} est introuvable")]
    RuntimeMissing {
        runtime: String,
        min: Option<String>,
    },
    #[error("{runtime} {found} est trop ancien")]
    RuntimeTooOld {
        runtime: String,
        found: String,
        min: String,
    },
    #[error("le jeu est déjà lancé")]
    AlreadyRunning,
    #[error("impossible de lancer le jeu : {0}")]
    Launch(String),
    #[error("aucune sauvegarde pour l'instant")]
    NoSaves,
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
            Self::UnknownVersion(_) => "unknownVersion",
            Self::InvalidCatalog(_) => "invalidCatalog",
            Self::TooLarge(_) => "tooLarge",
            Self::Unavailable => "unavailable",
            Self::MissingDigest => "missingDigest",
            Self::DigestMismatch => "digestMismatch",
            Self::BadArchive(_) => "badArchive",
            Self::Cancelled => "cancelled",
            Self::Busy => "busy",
            Self::NotInstalled => "notInstalled",
            Self::RuntimeMissing { .. } => "runtimeMissing",
            Self::RuntimeTooOld { .. } => "runtimeTooOld",
            Self::AlreadyRunning => "alreadyRunning",
            Self::Launch(_) => "launch",
            Self::NoSaves => "noSaves",
            Self::Io(_) => "io",
        }
    }

    fn data(&self) -> Value {
        match self {
            Self::RateLimited { reset } => json!({ "reset": reset }),
            Self::RuntimeMissing { runtime, min } => json!({ "runtime": runtime, "min": min }),
            Self::RuntimeTooOld {
                runtime,
                found,
                min,
            } => json!({ "runtime": runtime, "found": found, "min": min }),
            _ => Value::Null,
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
        state.serialize_field("data", &self.data())?;
        state.end()
    }
}
