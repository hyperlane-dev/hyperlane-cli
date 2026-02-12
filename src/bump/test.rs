use crate::*;

#[test]
fn test_bump_version_type_enum() {
    assert_eq!(BumpVersionType::Patch, BumpVersionType::Patch);
    assert_eq!(BumpVersionType::Minor, BumpVersionType::Minor);
    assert_eq!(BumpVersionType::Major, BumpVersionType::Major);
    assert_eq!(BumpVersionType::Release, BumpVersionType::Release);
    assert_eq!(BumpVersionType::Alpha, BumpVersionType::Alpha);
    assert_eq!(BumpVersionType::Beta, BumpVersionType::Beta);
    assert_eq!(BumpVersionType::Rc, BumpVersionType::Rc);
}

#[test]
fn test_version_struct_creation() {
    let version: Version = Version {
        major: 1,
        minor: 2,
        patch: 3,
        prerelease: Some("alpha.1".to_string()),
    };
    assert_eq!(version.major, 1);
    assert_eq!(version.minor, 2);
    assert_eq!(version.patch, 3);
    assert_eq!(version.prerelease, Some("alpha.1".to_string()));
}

#[test]
fn test_version_clone() {
    let version: Version = Version {
        major: 1,
        minor: 2,
        patch: 3,
        prerelease: Some("beta".to_string()),
    };
    let cloned: Version = version.clone();
    assert_eq!(cloned.major, version.major);
    assert_eq!(cloned.minor, version.minor);
    assert_eq!(cloned.patch, version.patch);
    assert_eq!(cloned.prerelease, version.prerelease);
}

#[test]
fn test_execute_bump_integration() {
    use std::fs::write;
    use std::path::PathBuf;
    let tmp_dir: PathBuf = PathBuf::from("./tmp/test_bump");
    let _ = std::fs::create_dir_all(&tmp_dir);
    let manifest_path: PathBuf = tmp_dir.join("Cargo.toml");
    let content: &str = r#"[package]
name = "test-package"
version = "0.1.0"
edition = "2024"
"#;
    write(&manifest_path, content).unwrap();
    let result: Result<String, Box<dyn std::error::Error>> =
        execute_bump(manifest_path.to_str().unwrap(), &BumpVersionType::Patch);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "0.1.1");
    let updated_content: String = std::fs::read_to_string(&manifest_path).unwrap();
    assert!(updated_content.contains("version = \"0.1.1\""));
}

#[test]
fn test_execute_bump_minor() {
    use std::fs::write;
    use std::path::PathBuf;
    let tmp_dir: PathBuf = PathBuf::from("./tmp/test_bump_minor");
    let _ = std::fs::create_dir_all(&tmp_dir);
    let manifest_path: PathBuf = tmp_dir.join("Cargo.toml");
    let content: &str = r#"[package]
name = "test-package"
version = "0.1.0"
edition = "2024"
"#;
    write(&manifest_path, content).unwrap();
    let result: Result<String, Box<dyn std::error::Error>> =
        execute_bump(manifest_path.to_str().unwrap(), &BumpVersionType::Minor);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "0.2.0");
}

#[test]
fn test_execute_bump_major() {
    use std::fs::write;
    use std::path::PathBuf;
    let tmp_dir: PathBuf = PathBuf::from("./tmp/test_bump_major");
    let _ = std::fs::create_dir_all(&tmp_dir);
    let manifest_path: PathBuf = tmp_dir.join("Cargo.toml");
    let content: &str = r#"[package]
name = "test-package"
version = "0.1.0"
edition = "2024"
"#;
    write(&manifest_path, content).unwrap();
    let result: Result<String, Box<dyn std::error::Error>> =
        execute_bump(manifest_path.to_str().unwrap(), &BumpVersionType::Major);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "1.0.0");
}

#[test]
fn test_execute_bump_alpha() {
    use std::fs::write;
    use std::path::PathBuf;
    let tmp_dir: PathBuf = PathBuf::from("./tmp/test_bump_alpha");
    let _ = std::fs::create_dir_all(&tmp_dir);
    let manifest_path: PathBuf = tmp_dir.join("Cargo.toml");
    let content: &str = r#"[package]
name = "test-package"
version = "0.1.0"
edition = "2024"
"#;
    write(&manifest_path, content).unwrap();
    let result: Result<String, Box<dyn std::error::Error>> =
        execute_bump(manifest_path.to_str().unwrap(), &BumpVersionType::Alpha);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "0.1.0-alpha");
}

#[test]
fn test_execute_bump_beta() {
    use std::fs::write;
    use std::path::PathBuf;
    let tmp_dir: PathBuf = PathBuf::from("./tmp/test_bump_beta");
    let _ = std::fs::create_dir_all(&tmp_dir);
    let manifest_path: PathBuf = tmp_dir.join("Cargo.toml");
    let content: &str = r#"[package]
name = "test-package"
version = "0.1.0-alpha.2"
edition = "2024"
"#;
    write(&manifest_path, content).unwrap();
    let result: Result<String, Box<dyn std::error::Error>> =
        execute_bump(manifest_path.to_str().unwrap(), &BumpVersionType::Beta);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "0.1.0-beta.1");
}

#[test]
fn test_execute_bump_rc() {
    use std::fs::write;
    use std::path::PathBuf;
    let tmp_dir: PathBuf = PathBuf::from("./tmp/test_bump_rc");
    let _ = std::fs::create_dir_all(&tmp_dir);
    let manifest_path: PathBuf = tmp_dir.join("Cargo.toml");
    let content: &str = r#"[package]
name = "test-package"
version = "0.1.0-beta.1"
edition = "2024"
"#;
    write(&manifest_path, content).unwrap();
    let result: Result<String, Box<dyn std::error::Error>> =
        execute_bump(manifest_path.to_str().unwrap(), &BumpVersionType::Rc);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "0.1.0-rc.1");
}

#[test]
fn test_execute_bump_release() {
    use std::fs::write;
    use std::path::PathBuf;
    let tmp_dir: PathBuf = PathBuf::from("./tmp/test_bump_release");
    let _ = std::fs::create_dir_all(&tmp_dir);
    let manifest_path: PathBuf = tmp_dir.join("Cargo.toml");
    let content: &str = r#"[package]
name = "test-package"
version = "0.1.0-alpha"
edition = "2024"
"#;
    write(&manifest_path, content).unwrap();
    let result: Result<String, Box<dyn std::error::Error>> =
        execute_bump(manifest_path.to_str().unwrap(), &BumpVersionType::Release);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "0.1.0");
}

#[test]
fn test_execute_bump_no_version_field() {
    use std::fs::write;
    use std::path::PathBuf;
    let tmp_dir: PathBuf = PathBuf::from("./tmp/test_bump_no_version");
    let _ = std::fs::create_dir_all(&tmp_dir);
    let manifest_path: PathBuf = tmp_dir.join("Cargo.toml");
    let content: &str = r#"[package]
name = "test-package"
edition = "2024"
"#;
    write(&manifest_path, content).unwrap();
    let result: Result<String, Box<dyn std::error::Error>> =
        execute_bump(manifest_path.to_str().unwrap(), &BumpVersionType::Patch);
    assert!(result.is_err());
}
