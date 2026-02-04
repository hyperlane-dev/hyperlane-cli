use crate::*;

/// Parse command line arguments
///
/// # Returns
///
/// - `Args`: Parsed arguments
pub fn parse_args() -> Args {
    let raw_args: Vec<String> = args().collect();
    let mut command: CommandType = CommandType::Help;
    let mut check: bool = false;
    let mut manifest_path: Option<String> = None;
    let mut i: usize = 1;
    while i < raw_args.len() {
        let arg: &str = raw_args[i].as_str();
        match arg {
            "-h" | "--help" => {
                command = CommandType::Help;
            }
            "-v" | "--version" => {
                command = CommandType::Version;
            }
            "fmt" => {
                if command == CommandType::Help || command == CommandType::Version {
                    command = CommandType::Fmt;
                }
            }
            "watch" => {
                if command == CommandType::Help || command == CommandType::Version {
                    command = CommandType::Watch;
                }
            }
            "--check" => {
                check = true;
            }
            "--manifest-path" => {
                i += 1;
                if i < raw_args.len() {
                    manifest_path = Some(raw_args[i].clone());
                }
            }
            _ => {}
        }
        i += 1;
    }
    Args {
        command,
        check,
        manifest_path,
    }
}
