use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

const BIN_NAME: &str = "tailrsc";
const TEST_DATA_DIR: &str = "src/tests/data";

fn test_data_path(filename: &str) -> String {
    Path::new(TEST_DATA_DIR)
        .join(filename)
        .to_string_lossy()
        .to_string()
}

#[test]
fn test_data_files_exist() {
    let files = ["empty.txt", "one.txt", "two.txt", "three.txt", "twelve.txt"];
    for file in &files {
        let path = test_data_path(file);
        assert!(Path::new(&path).exists(), "Test data file missing: {path}");
    }
}

#[test]
fn cli_no_args_succeeds() {
    let mut cmd = Command::cargo_bin(BIN_NAME).unwrap();
    cmd.assert().success();
}

#[test]
fn cli_version() {
    let mut cmd = Command::cargo_bin(BIN_NAME).unwrap();
    cmd.arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains(env!("CARGO_PKG_VERSION")));
}

#[test]
fn cli_help() {
    let mut cmd = Command::cargo_bin(BIN_NAME).unwrap();
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Display the last part of a file"));
}

// ---- Invalid -n argument tests ----

#[test]
fn cli_invalid_n_text() {
    Command::cargo_bin(BIN_NAME)
        .unwrap()
        .args(["-n", "test"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("illegal offset -- test"));
}

#[test]
fn cli_invalid_n_double_plus() {
    Command::cargo_bin(BIN_NAME)
        .unwrap()
        .args(["-n", "++5"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("illegal offset -- ++5"));
}

#[test]
fn cli_invalid_n_bad_suffix() {
    Command::cargo_bin(BIN_NAME)
        .unwrap()
        .args(["-n", "12abc"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("illegal offset -- 12abc"));
}

#[test]
fn cli_invalid_n_empty() {
    Command::cargo_bin(BIN_NAME)
        .unwrap()
        .args(["-n", ""])
        .assert()
        .failure()
        .stderr(predicate::str::contains("illegal offset"));
}

// ---- Invalid -c argument tests ----

#[test]
fn cli_invalid_c_text() {
    Command::cargo_bin(BIN_NAME)
        .unwrap()
        .args(["-c", "test"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("illegal offset -- test"));
}

#[test]
fn cli_invalid_c_sign_only() {
    Command::cargo_bin(BIN_NAME)
        .unwrap()
        .args(["-c", "+"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("illegal offset -- +"));
}

// ---- Valid argument tests ----

#[test]
fn cli_valid_n_explicit() {
    Command::cargo_bin(BIN_NAME)
        .unwrap()
        .args(["-n", "5"])
        .assert()
        .success();
}

#[test]
fn cli_valid_n_plus() {
    Command::cargo_bin(BIN_NAME)
        .unwrap()
        .args(["-n", "+5"])
        .assert()
        .success();
}

#[test]
fn cli_valid_n_minus() {
    Command::cargo_bin(BIN_NAME)
        .unwrap()
        .args(["-n", "-5"])
        .assert()
        .success();
}

#[test]
fn cli_valid_c_suffix() {
    Command::cargo_bin(BIN_NAME)
        .unwrap()
        .args(["-c", "5k"])
        .assert()
        .success();
}

#[test]
fn cli_valid_c_plus_suffix() {
    Command::cargo_bin(BIN_NAME)
        .unwrap()
        .args(["-c", "+3b"])
        .assert()
        .success();
}

// ---- File opening tests ----

#[test]
fn cli_single_valid_file_succeeds() {
    Command::cargo_bin(BIN_NAME)
        .unwrap()
        .arg(test_data_path("one.txt"))
        .assert()
        .success()
        .stderr(predicate::str::is_empty());
}

#[test]
fn cli_single_nonexistent_file_fails() {
    Command::cargo_bin(BIN_NAME)
        .unwrap()
        .arg("nonexistent.txt")
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "tailrsc: nonexistent.txt: No such file or directory",
        ));
}

#[test]
fn cli_multiple_nonexistent_files_reports_all() {
    Command::cargo_bin(BIN_NAME)
        .unwrap()
        .args(["bad1.txt", "bad2.txt"])
        .assert()
        .failure()
        .stderr(
            predicate::str::contains("tailrsc: bad1.txt:")
                .and(predicate::str::contains("tailrsc: bad2.txt:")),
        );
}

#[test]
fn cli_mixed_valid_and_invalid_files() {
    Command::cargo_bin(BIN_NAME)
        .unwrap()
        .args([&test_data_path("one.txt"), "nonexistent.txt"])
        .assert()
        .failure()
        .stderr(
            predicate::str::contains("tailrsc: nonexistent.txt:")
                .and(predicate::str::contains("one.txt").not()),
        );
}

#[test]
fn cli_permission_denied() {
    // Skip this test when running as root (e.g. in CI containers)
    if std::env::var("USER").unwrap_or_default() == "root" {
        return;
    }

    let path = Path::new(TEST_DATA_DIR).join("no_read.tmp");

    // Ensure clean slate if a prior run left the file behind
    if path.exists() {
        let _ = fs::set_permissions(&path, fs::Permissions::from_mode(0o644));
        let _ = fs::remove_file(&path);
    }

    fs::write(&path, "secret").unwrap();
    fs::set_permissions(&path, fs::Permissions::from_mode(0o000)).unwrap();

    let result = Command::cargo_bin(BIN_NAME)
        .unwrap()
        .arg(path.to_str().unwrap())
        .assert()
        .failure()
        .stderr(predicate::str::contains("Permission denied"));

    // Cleanup
    fs::set_permissions(&path, fs::Permissions::from_mode(0o644)).unwrap();
    fs::remove_file(&path).unwrap();

    drop(result);
}
