use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

// alias result
type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn no_args_dies() -> TestResult {
    // use ? instead of Result::unwrap to :
    // - unpack an Ok value or
    // - propagate an Err
    Command::cargo_bin("echor")?
        .assert()
        .failure()
        .stderr(predicate::str::contains("USAGE"));
    Ok(())
}

// Rust has many types of "string"
// - & indicates we only want to borrow for this scope
// - args : slice of &str
// - expected_file will be &str
// - returns TestResult
fn run(args: &[&str], expected_file: &str) -> TestResult {
    // try to read contents of expected_file into a string
    let expected = fs::read_to_string(expected_file)?;
    // Attempts to run echor in current crate with given args
    // - assert that STDOUT == expected value
    Command::cargo_bin("echor")?
        .args(args)
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}

#[test]
fn args_1() -> TestResult {
    run(&["Hello there"], "tests/expected/args_1.txt")
}

#[test]
fn args_2() -> TestResult {
    run(&["Hello", "there"], "tests/expected/args_2.txt")
}

#[test]
fn args_1_no_newline() -> TestResult {
    run(&["Hello  there", "-n"], "tests/expected/args_1.n.txt")
}

#[test]
fn args_2_no_newline() -> TestResult {
    return run(&["-n", "Hello", "there"], "tests/expected/args_2.n.txt");
}
