use crate::*;

impl std::fmt::Display for PublishError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PublishError::ManifestParseError => write!(f, "failed to parse Cargo.toml"),
            PublishError::CircularDependency => write!(f, "circular dependency detected"),
            PublishError::IoError => write!(f, "io error"),
        }
    }
}

impl std::error::Error for PublishError {}
