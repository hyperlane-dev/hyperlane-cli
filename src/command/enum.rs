/// Available commands
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum CommandType {
    /// Format code using cargo fmt
    Fmt,
    /// Watch files using cargo-watch
    Watch,
    /// Bump version in Cargo.toml
    Bump,
    /// Show help
    Help,
    /// Show version
    Version,
}
