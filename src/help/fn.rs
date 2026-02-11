/// Print help message
pub(crate) fn print_help() {
    println!("hyperlane-cli [COMMAND] [OPTIONS]");
    println!();
    println!("Commands:");
    println!("  fmt       Format Rust code using cargo fmt");
    println!("  watch     Watch files and run cargo run using cargo-watch");
    println!("  -h, --help      Print this help message");
    println!("  -v, --version   Print version information");
    println!();
    println!("Fmt Options:");
    println!("  --check         Check formatting without making changes");
    println!("  --manifest-path <PATH>  Path to Cargo.toml");
}
