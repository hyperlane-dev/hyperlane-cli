/// Available commands
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CommandType {
    /// Format code using cargo fmt
    Fmt,
    /// Watch files using cargo-watch
    Watch,
    /// Show help
    Help,
    /// Show version
    Version,
}
