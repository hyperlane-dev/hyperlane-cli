use crate::*;

/// Parsed command line arguments
#[derive(Clone, Debug)]
pub struct Args {
    /// The command to execute
    pub command: CommandType,
    /// Check mode for fmt
    pub check: bool,
    /// Manifest path for fmt and bump
    pub manifest_path: Option<String>,
    /// Bump type for bump command
    pub bump_type: Option<BumpVersionType>,
}
