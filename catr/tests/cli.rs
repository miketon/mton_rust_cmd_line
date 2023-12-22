use assert_cmd::Command;
use predicates::prelude::*;
use rand::{
    distributions::Alphanumeric,
    Rng
};
use std::error::Error;
use std::fs;

type TestResult = Result<(), Box<dyn Error>>;

const PRG: &str = "catr";
const EMPTY: &str = "tests/inputs/empty.txt";
const FOX: &str = "tests/inputs/fox.txt";
const SPIDERS: &str = "tests/inputs/spiders.txt";
const BUSTLE: &str = "tests/inputs/the-bustle.txt";

// ----------------------------------------------------------------------------
fn run() -> TestResult {
    Ok(())
}

// ----------------------------------------------------------------------------
fn gen_bad_file() -> String{
    loop {
        let filename: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            .map(char::from)
            .collect();

        if fs::metadata(&filename).is_err() {
            return filename
        }
    }
}

// ----------------------------------------------------------------------------
#[test]
fn usage() -> TestResult {
    for flag in &["-h","--help"]{
        Command::cargo_bin(PRG)?
            .arg(flag)
            .assert()
            .stdout(predicate::str::contains("USAGE"));
    }
    Ok(())
}
