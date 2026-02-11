/// Error types for publish operation
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum PublishError {
    /// Failed to parse Cargo.toml
    ManifestParseError,
    /// Circular dependency detected
    CircularDependency,
    /// IO error
    IoError,
}
