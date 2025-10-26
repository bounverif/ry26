use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_cli_help() {
    let mut cmd = Command::cargo_bin("ry26").unwrap();
    cmd.arg("--help");
    cmd.assert().success().stdout(predicate::str::contains(
        "A command line interface for the ry26 library",
    ));
}

#[test]
fn test_cli_add() {
    let mut cmd = Command::cargo_bin("ry26").unwrap();
    cmd.arg("add").arg("5").arg("10");
    cmd.assert().success().stdout("15\n");
}

#[test]
fn test_cli_add_zero() {
    let mut cmd = Command::cargo_bin("ry26").unwrap();
    cmd.arg("add").arg("0").arg("0");
    cmd.assert().success().stdout("0\n");
}

#[test]
fn test_cli_add_large_numbers() {
    let mut cmd = Command::cargo_bin("ry26").unwrap();
    cmd.arg("add").arg("1000000").arg("2000000");
    cmd.assert().success().stdout("3000000\n");
}

#[test]
fn test_cli_generate() {
    let mut cmd = Command::cargo_bin("ry26").unwrap();
    cmd.arg("generate");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("\"id\":"))
        .stdout(predicate::str::contains("\"value\":"))
        .stdout(predicate::str::contains("\"timestamp\":"));
}

#[test]
fn test_cli_to_json() {
    let mut cmd = Command::cargo_bin("ry26").unwrap();
    cmd.arg("to-json")
        .arg("--id")
        .arg("42")
        .arg("--value")
        .arg("3.14")
        .arg("--timestamp")
        .arg("2025-10-26T12:00:00Z");
    cmd.assert()
        .success()
        .stdout("{\"id\":42,\"value\":3.14,\"timestamp\":\"2025-10-26T12:00:00Z\"}\n");
}

#[test]
fn test_cli_from_json() {
    let mut cmd = Command::cargo_bin("ry26").unwrap();
    cmd.arg("from-json")
        .arg("{\"id\":100,\"value\":50.5,\"timestamp\":\"2025-10-26T15:30:00Z\"}");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("ID: 100"))
        .stdout(predicate::str::contains("Value: 50.5"))
        .stdout(predicate::str::contains("Timestamp: 2025-10-26T15:30:00Z"));
}

#[test]
fn test_cli_from_json_invalid() {
    let mut cmd = Command::cargo_bin("ry26").unwrap();
    cmd.arg("from-json").arg("{invalid json}");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Error:"));
}

#[test]
fn test_cli_from_json_incomplete() {
    let mut cmd = Command::cargo_bin("ry26").unwrap();
    cmd.arg("from-json").arg("{\"id\": 1}");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Error:"));
}

#[test]
fn test_cli_add_help() {
    let mut cmd = Command::cargo_bin("ry26").unwrap();
    cmd.arg("add").arg("--help");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Add two numbers together"))
        .stdout(predicate::str::contains("First number"))
        .stdout(predicate::str::contains("Second number"));
}

#[test]
fn test_cli_generate_help() {
    let mut cmd = Command::cargo_bin("ry26").unwrap();
    cmd.arg("generate").arg("--help");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Generate a random data point"));
}

#[test]
fn test_cli_to_json_help() {
    let mut cmd = Command::cargo_bin("ry26").unwrap();
    cmd.arg("to-json").arg("--help");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Convert data point to JSON"))
        .stdout(predicate::str::contains("--id"))
        .stdout(predicate::str::contains("--value"))
        .stdout(predicate::str::contains("--timestamp"));
}

#[test]
fn test_cli_from_json_help() {
    let mut cmd = Command::cargo_bin("ry26").unwrap();
    cmd.arg("from-json").arg("--help");
    cmd.assert().success().stdout(predicate::str::contains(
        "Parse JSON string and display data point",
    ));
}
