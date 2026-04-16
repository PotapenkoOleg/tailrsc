use assert_cmd::Command;
use predicates::prelude::*;
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
