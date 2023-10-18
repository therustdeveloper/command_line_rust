use assert_cmd::Command;
use predicates::prelude::*;
use rand::{distributions::Alphanumeric, Rng};
use std::fs;

// type used by echor
type TestResult = Result<(), Box<dyn std::error::Error>>;

const PRG: &str = "command_line_rust";
const EMPTY: &str = "tests/inputs/empty.txt";
const FOX: &str = "tests/inputs/fox.txt";
const SPIDERS: &str = "tests/inputs/spiders.txt";
const BUSTLE: &str = "tests/inputs/the-bustle.txt";

#[test]
#[ignore]
fn hello_runs() {
    let mut cmd = Command::cargo_bin(PRG).unwrap();
    cmd.assert().success();
}

#[test]
#[ignore]
fn hello_output() {
    let mut cmd = Command::cargo_bin(PRG).unwrap();
    cmd.assert().success().stdout("Hello, world!\n");
}

#[test]
#[ignore]
fn echor_dies_no_args() -> TestResult {
    Command::cargo_bin(PRG)?
        .assert()
        .failure()
        .stderr(predicate::str::contains("USAGE"));
    Ok(())
}

// --------------------------------------------------
fn run(args: &[&str], expected_file: &str) -> TestResult {
    let expected = fs::read_to_string(expected_file)?;
    Command::cargo_bin(PRG)?
        .args(args)
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}

#[test]
#[ignore]
fn echor_hello1() -> TestResult {
    run(&["Hello there"], "tests/expected/hello1.txt")
}

#[test]
#[ignore]
fn echor_hello2() -> TestResult {
    run(&["Hello", "there"], "tests/expected/hello2.txt")
}

#[test]
#[ignore]
fn echor_hello1_no_newline() -> TestResult {
    run(&["Hello  there", "-n"], "tests/expected/hello1.n.txt")
}

#[test]
#[ignore]
fn echor_hello2_no_newline() -> TestResult {
    run(&["-n", "Hello", "there"], "tests/expected/hello2.n.txt")
}

// --------------------------------------------------
#[test]
#[ignore]
fn catr_usage() -> TestResult {
    for flag in &["-h", "--help"] {
        Command::cargo_bin(PRG)?
            .arg(flag)
            .assert()
            .stdout(predicate::str::contains("USAGE"));
    }
    Ok(())
}

// --------------------------------------------------
fn gen_bad_file() -> String {
    loop {
        let filename: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            .map(char::from)
            .collect();

        if fs::metadata(&filename).is_err() {
            return filename;
        }
    }
}

// --------------------------------------------------
#[test]
#[ignore]
fn catr_skips_bad_file() -> TestResult {
    let bad = gen_bad_file();
    let expected = format!("{}: .* [(]os error 2[)]", bad);
    Command::cargo_bin(PRG)?
        .arg(&bad)
        .assert()
        .success()
        .stderr(predicate::str::is_match(expected)?);
    Ok(())
}

// --------------------------------------------------
fn run_stdin(input_file: &str, args: &[&str], expected_file: &str) -> TestResult {
    let input = fs::read_to_string(input_file)?;
    let expected = fs::read_to_string(expected_file)?;
    Command::cargo_bin(PRG)?
        .args(args)
        .write_stdin(input)
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}

// --------------------------------------------------
#[test]
#[ignore]
fn catr_bustle_stdin() -> TestResult {
    run_stdin(BUSTLE, &["-"], "tests/expected/the-bustle.txt.stdin.out")
}

// --------------------------------------------------
#[test]
#[ignore]
fn catr_bustle_stdin_n() -> TestResult {
    run_stdin(
        BUSTLE,
        &["-n", "-"],
        "tests/expected/the-bustle.txt.n.stdin.out",
    )
}

// --------------------------------------------------
#[test]
#[ignore]
fn catr_bustle_stdin_b() -> TestResult {
    run_stdin(
        BUSTLE,
        &["-b", "-"],
        "tests/expected/the-bustle.txt.b.stdin.out",
    )
}

// --------------------------------------------------
#[test]
#[ignore]
fn catr_empty() -> TestResult {
    run(&[EMPTY], "tests/expected/empty.txt.out")
}

// --------------------------------------------------
#[test]
#[ignore]
fn catr_empty_n() -> TestResult {
    run(&["-n", EMPTY], "tests/expected/empty.txt.n.out")
}

// --------------------------------------------------
#[test]
#[ignore]
fn catr_empty_b() -> TestResult {
    run(&["-b", EMPTY], "tests/expected/empty.txt.b.out")
}

// --------------------------------------------------
#[test]
#[ignore]
fn catr_fox() -> TestResult {
    run(&[FOX], "tests/expected/fox.txt.out")
}

// --------------------------------------------------
#[test]
#[ignore]
fn catr_fox_n() -> TestResult {
    run(&["-n", FOX], "tests/expected/fox.txt.n.out")
}

// --------------------------------------------------
#[test]
#[ignore]
fn catr_fox_b() -> TestResult {
    run(&["-b", FOX], "tests/expected/fox.txt.b.out")
}

// --------------------------------------------------
#[test]
#[ignore]
fn catr_spiders() -> TestResult {
    run(&[SPIDERS], "tests/expected/spiders.txt.out")
}

// --------------------------------------------------
#[test]
#[ignore]
fn catr_spiders_n() -> TestResult {
    run(&["--number", SPIDERS], "tests/expected/spiders.txt.n.out")
}

// --------------------------------------------------
#[test]
#[ignore]
fn catr_spiders_b() -> TestResult {
    run(
        &["--number-nonblank", SPIDERS],
        "tests/expected/spiders.txt.b.out",
    )
}

// --------------------------------------------------
#[test]
#[ignore]
fn catr_bustle() -> TestResult {
    run(&[BUSTLE], "tests/expected/the-bustle.txt.out")
}

// --------------------------------------------------
#[test]
#[ignore]
fn catr_bustle_n() -> TestResult {
    run(&["-n", BUSTLE], "tests/expected/the-bustle.txt.n.out")
}

// --------------------------------------------------
#[test]
#[ignore]
fn catr_bustle_b() -> TestResult {
    run(&["-b", BUSTLE], "tests/expected/the-bustle.txt.b.out")
}

// --------------------------------------------------
#[test]
#[ignore]
fn catr_all() -> TestResult {
    run(&[FOX, SPIDERS, BUSTLE], "tests/expected/all.out")
}

// --------------------------------------------------
#[test]
#[ignore]
fn catr_all_n() -> TestResult {
    run(&[FOX, SPIDERS, BUSTLE, "-n"], "tests/expected/all.n.out")
}

// --------------------------------------------------
#[test]
#[ignore]
fn catr_all_b() -> TestResult {
    run(&[FOX, SPIDERS, BUSTLE, "-b"], "tests/expected/all.b.out")
}
