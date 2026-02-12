use crate::*;

#[test]
fn test_format_path_integration() {
    use std::path::PathBuf;
    let tmp_dir: PathBuf = PathBuf::from("./tmp/test_fmt");
    let _ = std::fs::create_dir_all(&tmp_dir);
    let test_file: PathBuf = tmp_dir.join("test.rs");
    std::fs::write(&test_file, "fn main() {\n    println!(\"hello\");\n}\n").unwrap();
    let rt: tokio::runtime::Runtime = tokio::runtime::Runtime::new().unwrap();
    let result: Result<(), std::io::Error> = rt.block_on(format_path(&tmp_dir));
    assert!(result.is_ok());
}
