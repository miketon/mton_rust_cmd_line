use assert_cmd::Command;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use predicates::prelude::*;
use rand::{Rng, distributions::Alphanumeric};

type TestResult = Result<(), Box<dyn Error>>;

const PRG: &str = "headr";
const EMPTY: &str = "./tests/inputs/empty.txt";

//---------------------------------------------------------------------------80

fn random_string() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(7)
        .map(char::from)
        .collect()
}

fn run(args: &[&str], expected_file: &str) -> TestResult {
    let mut file = File::open(expected_file)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    let expected = String::from_utf8_lossy(&buffer);

    Command::cargo_bin(PRG)?
        .args(args)
        .assert()
        .success()
        .stdout(predicate::eq(&expected.as_bytes() as &[u8]));

    Ok(())
}

//---------------------------------------------------------------------------80

#[test]
fn dies_bad_bytes() -> TestResult {
    let bad = random_string();
    let expected = format!("illegal byte count -- {}", &bad);
    Command::cargo_bin(PRG)?
        .args(&["-c", &bad, EMPTY])
        .assert()
        .failure()
        .stderr(predicate::str::contains(expected));

    Ok(())
}

#[test]
fn dies_bad_lines() -> TestResult {
    let bad = random_string();
    let expected = format!("illegal line count -- {}", &bad);
    Command::cargo_bin(PRG)?
        .args(&["-n", &bad, EMPTY])
        .assert()
        .failure()
        .stderr(predicate::str::contains(expected));

    Ok(())
}

#[test]
fn dies_bytes_and_lines() -> TestResult {
    let bad = random_string();
    let expected = format!("'--lines <LINES>' cannot be used with '--bytes <BYTES>'");
    Command::cargo_bin(PRG)?
        .args(&["-c", &bad, "-n", &bad, EMPTY])
        .assert()
        .failure()
        .stderr(predicate::str::contains(expected));

    Ok(())
}

#[test]
fn dies_lines_and_bytes() -> TestResult {
    let bad = random_string();
    let expected = format!("'--lines <LINES>' cannot be used with '--bytes <BYTES>'");
    Command::cargo_bin(PRG)?
        .args(&["-n", &bad, "-c", &bad, EMPTY])
        .assert()
        .failure()
        .stderr(predicate::str::contains(expected));

    Ok(())
}

//---------------------------------------------------------------------------80

#[test]
fn empty() -> TestResult {
    run(&[EMPTY], "tests/expected/empty.txt.out")
}
