/// Print help message
pub(crate) fn print_help() {
    println!("hyperlane-cli [COMMAND] [OPTIONS]");
    println!();
    println!("Commands:");
    println!("  bump      Bump version in Cargo.toml");
    println!("  fmt       Format Rust code using cargo fmt");
    println!("  watch     Watch files and run cargo run using cargo-watch");
    println!("  publish   Publish packages in monorepo with topological ordering");
    println!("  new       Create a new project from template");
    println!(
        "  template  Generate template components (controller|domain|exception|mapper|model|repository|service|utils|view)"
    );
    println!("  -h, --help      Print this help message");
    println!("  -v, --version   Print version information");
    println!();
    println!("New Options:");
    println!("  <PROJECT_NAME>  Name of the project to create");
    println!();
    println!("Bump Options:");
    println!("  --patch         Bump patch version (0.1.0 -> 0.1.1) [default]");
    println!("  --minor         Bump minor version (0.1.0 -> 0.2.0)");
    println!("  --major         Bump major version (0.1.0 -> 1.0.0)");
    println!(
        "  --alpha         Add or bump alpha version (0.1.0 -> 0.1.0-alpha, 0.1.0-alpha -> 0.1.0-alpha.1)"
    );
    println!(
        "  --beta          Add or bump beta version (0.1.0 -> 0.1.0-beta, 0.1.0-alpha.2 -> 0.1.0-beta.1)"
    );
    println!(
        "  --rc            Add or bump rc version (0.1.0 -> 0.1.0-rc, 0.1.0-beta.1 -> 0.1.0-rc.1)"
    );
    println!("  --release       Remove pre-release identifier (0.1.0-alpha -> 0.1.0)");
    println!("  --manifest-path <PATH>  Path to Cargo.toml [default: Cargo.toml]");
    println!();
    println!("Fmt Options:");
    println!("  --check         Check formatting without making changes");
    println!("  --manifest-path <PATH>  Path to Cargo.toml");
    println!();
    println!("Publish Options:");
    println!("  --manifest-path <PATH>  Path to workspace Cargo.toml [default: Cargo.toml]");
    println!("  --max-retries <N>       Maximum retry attempts per package [default: 3]");
}
