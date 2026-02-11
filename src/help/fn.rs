/// Print help message
pub(crate) fn print_help() {
    println!("hyperlane-cli [COMMAND] [OPTIONS]");
    println!();
    println!("Commands:");
    println!("  bump      Bump version in Cargo.toml");
    println!("  fmt       Format Rust code using cargo fmt");
    println!("  watch     Watch files and run cargo run using cargo-watch");
    println!("  -h, --help      Print this help message");
    println!("  -v, --version   Print version information");
    println!();
    println!("Bump Options:");
    println!("  --patch         Bump patch version (0.1.2 -> 0.1.3) [default]");
    println!("  --minor         Bump minor version (0.1.2 -> 0.2.0)");
    println!("  --major         Bump major version (0.1.2 -> 1.0.0)");
    println!(
        "  --alpha         Add or bump alpha version (0.1.2 -> 0.1.2-alpha, 0.1.2-alpha -> 0.1.2-alpha.1)"
    );
    println!(
        "  --beta          Add or bump beta version (0.1.2 -> 0.1.2-beta, 0.1.2-alpha.2 -> 0.1.2-beta.1)"
    );
    println!(
        "  --rc            Add or bump rc version (0.1.2 -> 0.1.2-rc, 0.1.2-beta.1 -> 0.1.2-rc.1)"
    );
    println!("  --release       Remove pre-release identifier (0.1.2-alpha -> 0.1.2)");
    println!("  --manifest-path <PATH>  Path to Cargo.toml [default: Cargo.toml]");
    println!();
    println!("Fmt Options:");
    println!("  --check         Check formatting without making changes");
    println!("  --manifest-path <PATH>  Path to Cargo.toml");
}
