/// Available commands
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum CommandType {
    /// Format code using cargo fmt
    Fmt,
    /// Watch files using cargo-watch
    Watch,
    /// Bump version in Cargo.toml
    Bump,
    /// Publish packages in monorepo
    Publish,
    /// Create a new project from template
    New,
    /// Show help
    Help,
    /// Show version
    Version,
}
