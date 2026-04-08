use assert_cmd::Command;
use serde_json::Value;
use std::fs;
use std::path::Path;
use tempfile::TempDir;

fn setup_project() -> TempDir {
    let temp = tempfile::tempdir().unwrap();
    fs::create_dir_all(temp.path().join("src/models")).unwrap();

    fs::write(
        temp.path().join("Cargo.toml"),
        r#"
[package]
name = "demo"
version = "0.1.0"
edition = "2021"
"#,
    )
    .unwrap();

    fs::write(
        temp.path().join("src/models/user.rs"),
        r#"
#[derive(TypeWriter)]
#[sync_to(typescript, python)]
pub struct User {
    pub id: String,
    pub email: String,
}
"#,
    )
    .unwrap();

    temp
}

fn run_typewriter(project: &Path, args: &[&str]) -> assert_cmd::assert::Assert {
    let mut cmd = Command::cargo_bin("typewriter").unwrap();
    cmd.current_dir(project).args(args).assert()
}

fn run_cargo_typewriter(project: &Path, args: &[&str]) -> assert_cmd::assert::Assert {
    let mut cmd = Command::cargo_bin("cargo-typewriter").unwrap();
    cmd.current_dir(project).args(args).assert()
}

#[test]
fn generate_all_writes_expected_files() {
    let temp = setup_project();

    run_typewriter(temp.path(), &["generate", "--all"]).success();

    assert!(temp.path().join("generated/typescript/user.ts").exists());
    assert!(
        temp.path()
            .join("generated/typescript/user.schema.ts")
            .exists()
    );
    assert!(temp.path().join("generated/python/user.py").exists());
}

#[test]
fn generate_single_file_lang_filter() {
    let temp = setup_project();

    run_typewriter(
        temp.path(),
        &["generate", "src/models/user.rs", "--lang", "typescript"],
    )
    .success();

    assert!(temp.path().join("generated/typescript/user.ts").exists());
    assert!(
        temp.path()
            .join("generated/typescript/user.schema.ts")
            .exists()
    );
    assert!(!temp.path().join("generated/python/user.py").exists());
}

#[test]
fn generate_respects_typescript_zod_config() {
    let temp = setup_project();

    fs::write(
        temp.path().join("typewriter.toml"),
        r#"
[typescript]
zod = false
"#,
    )
    .unwrap();

    run_typewriter(temp.path(), &["generate", "--all"]).success();

    assert!(temp.path().join("generated/typescript/user.ts").exists());
    assert!(
        !temp
            .path()
            .join("generated/typescript/user.schema.ts")
            .exists()
    );
    assert!(temp.path().join("generated/python/user.py").exists());
}

#[test]
fn generate_respects_per_type_tw_zod_override() {
    let temp = setup_project();

    fs::write(
        temp.path().join("typewriter.toml"),
        r#"
[typescript]
zod = false
"#,
    )
    .unwrap();

    fs::write(
        temp.path().join("src/models/user.rs"),
        r#"
#[derive(TypeWriter)]
#[sync_to(typescript)]
pub struct UserProfile {
    pub id: String,
}

#[derive(TypeWriter)]
#[sync_to(typescript)]
#[tw(zod)]
pub struct Address {
    pub city: String,
}

#[derive(TypeWriter)]
#[sync_to(typescript)]
pub struct Order {
    pub id: String,
}
"#,
    )
    .unwrap();

    run_typewriter(temp.path(), &["generate", "--all"]).success();

    assert!(
        temp.path()
            .join("generated/typescript/user-profile.ts")
            .exists()
    );
    assert!(temp.path().join("generated/typescript/address.ts").exists());
    assert!(temp.path().join("generated/typescript/order.ts").exists());

    assert!(
        !temp
            .path()
            .join("generated/typescript/user-profile.schema.ts")
            .exists()
    );
    assert!(
        temp.path()
            .join("generated/typescript/address.schema.ts")
            .exists()
    );
    assert!(
        !temp
            .path()
            .join("generated/typescript/order.schema.ts")
            .exists()
    );

    run_typewriter(temp.path(), &["check", "--ci"]).success();
}

#[test]
fn generate_diff_prints_unified_diff() {
    let temp = setup_project();

    run_typewriter(temp.path(), &["generate", "--all"]).success();

    let ts_path = temp.path().join("generated/typescript/user.ts");
    fs::write(&ts_path, "// changed\n").unwrap();

    run_typewriter(temp.path(), &["generate", "--all", "--diff"])
        .success()
        .stdout(predicates::str::contains(
            "--- a/generated/typescript/user.ts",
        ));
}

#[test]
fn check_detects_schema_drift_and_ci_fails() {
    let temp = setup_project();

    fs::write(
        temp.path().join("src/models/order.rs"),
        r#"
#[derive(TypeWriter)]
#[sync_to(typescript)]
pub struct Order {
    pub id: String,
}
"#,
    )
    .unwrap();

    run_typewriter(temp.path(), &["generate", "--all"]).success();

    let user_schema_path = temp.path().join("generated/typescript/user.schema.ts");
    fs::write(&user_schema_path, "// changed schema\n").unwrap();

    let order_schema_path = temp.path().join("generated/typescript/order.schema.ts");
    fs::remove_file(&order_schema_path).unwrap();

    let orphan_schema_path = temp.path().join("generated/typescript/orphan.schema.ts");
    fs::create_dir_all(orphan_schema_path.parent().unwrap()).unwrap();
    fs::write(
        &orphan_schema_path,
        "// Auto-generated by typewriter v0.3.0. DO NOT EDIT.\n",
    )
    .unwrap();

    let output = Command::cargo_bin("typewriter")
        .unwrap()
        .current_dir(temp.path())
        .args(["check", "--json"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let report: Value = serde_json::from_slice(&output).unwrap();
    let entries = report["entries"].as_array().unwrap();

    let status_for = |path: &str| {
        entries
            .iter()
            .find(|entry| entry["output_path"].as_str() == Some(path))
            .and_then(|entry| entry["status"].as_str())
    };

    assert_eq!(
        status_for("generated/typescript/user.schema.ts"),
        Some("out_of_sync")
    );
    assert_eq!(
        status_for("generated/typescript/order.schema.ts"),
        Some("missing")
    );
    assert_eq!(
        status_for("generated/typescript/orphan.schema.ts"),
        Some("orphaned")
    );

    run_typewriter(temp.path(), &["check", "--ci"]).failure();
}

#[test]
fn check_json_out_writes_report_file() {
    let temp = setup_project();
    run_typewriter(temp.path(), &["generate", "--all"]).success();

    run_typewriter(
        temp.path(),
        &["check", "--json", "--json-out", "reports/drift.json"],
    )
    .success();

    let report_path = temp.path().join("reports/drift.json");
    assert!(report_path.exists());

    let parsed: Value = serde_json::from_slice(&fs::read(report_path).unwrap()).unwrap();
    assert!(parsed["entries"].is_array());
}

#[test]
fn cargo_typewriter_matches_generate_command() {
    let temp = setup_project();

    run_cargo_typewriter(temp.path(), &["generate", "--all"]).success();

    assert!(temp.path().join("generated/typescript/user.ts").exists());
    assert!(
        temp.path()
            .join("generated/typescript/user.schema.ts")
            .exists()
    );
    assert!(temp.path().join("generated/python/user.py").exists());
}
