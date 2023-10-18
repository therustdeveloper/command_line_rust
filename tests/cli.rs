use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

// type used by echor
type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
#[ignore]
fn hello_runs() {
    let mut cmd = Command::cargo_bin("command_line_rust").unwrap();
    cmd.assert().success();
}

#[test]
#[ignore]
fn hello_output() {
    let mut cmd = Command::cargo_bin("command_line_rust").unwrap();
    cmd.assert().success().stdout("Hello, world!\n");
}

#[test]
#[ignore]
fn echor_dies_no_args() -> TestResult {
    Command::cargo_bin("command_line_rust")?
        .assert()
        .failure()
        .stderr(predicate::str::contains("USAGE"));
    Ok(())
}

fn echor_runs(args: &[&str], expected_file: &str) -> TestResult {
    let expected = fs::read_to_string(expected_file)?;
    Command::cargo_bin("command_line_rust")?
        .args(args)
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}

#[test]
#[ignore]
fn echor_hello1() -> TestResult {
    echor_runs(&["Hello there"], "tests/expected/hello1.txt")
}

#[test]
#[ignore]
fn echor_hello2() -> TestResult {
    echor_runs(&["Hello", "there"], "tests/expected/hello2.txt")
}

#[test]
#[ignore]
fn echor_hello1_no_newline() -> TestResult {
    echor_runs(&["Hello  there", "-n"], "tests/expected/hello1.n.txt")
}

#[test]
#[ignore]
fn echor_hello2_no_newline() -> TestResult {
    echor_runs(&["-n", "Hello", "there"], "tests/expected/hello2.n.txt")
}
