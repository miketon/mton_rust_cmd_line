use assert_cmd::{cargo::CommandCargoExt, Command};
use predicates::prelude::*;
use rand::{distributions::Alphanumeric, Rng};
use std::fs;

type TestResult = Result<(), Box<dyn std::error::Error>>;

const PRG: &str = "wcr";
const EMPTY: &str = "tests/inputs/empty.txt";
const FOX: &str = "tests/inputs/fox.txt";
const ATLAMAL: &str = "tests/inputs/atlamal.txt";

// --------------------------------------------------------------------------80

fn run(args: &[&str], expected_file: &str) -> TestResult {
    let expected = fs::read_to_string(expected_file)?;

    Command::cargo_bin(PRG)?
        .args(args)
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}

fn gen_bad_file() -> String {
    loop {
        let filename = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            // @audit : Explain what is happening here
            .map(char::from)
            .collect();

        if fs::metadata(&filename).is_err() {
            return filename;
        }
    }
}

// --------------------------------------------------------------------------80
#[test]
fn dies_chars_and_bytes() -> TestResult {
    Command::cargo_bin(PRG)?
        .args(&["-m", "-c"])
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "The argument '--bytes' cannot be used with '--chars'",
        ));

    Ok(())
}

#[test]
fn empty() -> TestResult {
    run(&[EMPTY], "tests/expected/empty.txt.out")
}

#[test]
fn fox() -> TestResult {
    run(&[FOX], "tests/expected/fox.txt.out")
}

#[test]
fn atlamal() -> TestResult {
    run(&[ATLAMAL], "tests/expected/atlamal.txt.out")
}

#[test]
fn test_all() -> TestResult {
    run(&[EMPTY, FOX, ATLAMAL], "tests/expected/all.out")
}

// test for default : lines, words and bytes
// - DEFAULT ORDER : lines, words, byte/characters

// test [flags] => -l : lines
// test [flags] => -c : bytes
// test [flags] => -w : words
// test [flags] => -m : characters
// test [flags] => -mc : bytes
// test [flags] => -cm : characters
// test [flags] => -cw | -wc : DEFAULT order regardless of flags
//
// test [read from STDIN] does NOT print a filename
//
// [multi-files] => [total] # lines | words | bytes for all inputs
// [file-error] => Nonexistent files note wraning to STDERR as files process
