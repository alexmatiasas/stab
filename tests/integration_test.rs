use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_cli_help() {
    Command::cargo_bin("stab")
        .unwrap()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("data"));
}

#[test]
fn test_data_read_table() {
    Command::cargo_bin("stab")
        .unwrap()
        .args(["data", "read", "tests/fixtures/sample.csv"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Ana"))
        .stdout(predicate::str::contains("CDMX"));
}

#[test]
fn test_data_read_json() {
    Command::cargo_bin("stab")
        .unwrap()
        .args([
            "data",
            "read",
            "tests/fixtures/sample.csv",
            "--output",
            "json",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"name\""))
        .stdout(predicate::str::contains("\"Ana\""));
}

#[test]
fn test_data_describe() {
    Command::cargo_bin("stab")
        .unwrap()
        .args(["data", "describe", "tests/fixtures/sample.csv"])
        .assert()
        .success()
        .stdout(predicate::str::contains("shape"))
        .stdout(predicate::str::contains("dtype"));
}

#[test]
fn test_data_read_invalid_file() {
    Command::cargo_bin("stab")
        .unwrap()
        .args(["data", "read", "tests/fixtures/nonexistent.csv"])
        .assert()
        .failure();
}
