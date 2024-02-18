use assert_cmd::Command;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use predicates::prelude::*;
use rand::{Rng, distributions::Alphanumeric};

type TestResult = Result<(), Box<dyn Error>>;

const PRG: &str = "headr";
const EMPTY: &str = "./tests/inputs/empty.txt";
const ONE: &str = "./tests/inputs/one.txt";
const TWO: &str = "./tests/inputs/two.txt";
const THREE: &str = "./tests/inputs/three.txt";
const TEN: &str = "./tests/inputs/ten.txt";

//---------------------------------------------------------------------------80

fn random_string() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(7)
        .map(char::from)
        .collect()
}

// - args:: an ref to string array because we want to handle filepath and 
// flag/arg
// - expected_file:: just a string ref because we just want to compare to 
// 'head' output result given path+arg
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

#[test]
fn one() -> TestResult {
    run(&[ONE], "tests/expected/one.txt.out")
}

#[test]
fn two() -> TestResult {
    run(&[TWO], "tests/expected/two.txt.out")
}

#[test]
fn three() -> TestResult {
    run(&[THREE], "tests/expected/three.txt.out")
}

#[test]
fn ten() -> TestResult {
    run(&[TEN], "tests/expected/ten.txt.out")
}

//---------------------------------------------------------------------------80

#[test]
fn empty_n2() -> TestResult {
    run(&[EMPTY, "-n", "2"], "tests/expected/empty.txt.n2.out")
}

#[test]
fn one_n2() -> TestResult {
    run(&[ONE, "-n", "2"], "tests/expected/one.txt.n2.out")
}

#[test]
fn two_n2() -> TestResult {
    run(&[TWO, "-n", "2"], "tests/expected/two.txt.n2.out")
}

#[test]
fn three_n2() -> TestResult {
    run(&[THREE, "-n", "2"], "tests/expected/three.txt.n2.out")
}

#[test]
fn ten_n2() -> TestResult {
    run(&[TEN, "-n", "2"], "tests/expected/ten.txt.n2.out")
}


//---------------------------------------------------------------------------80

#[test]
fn empty_c1() -> TestResult {
    run(&[EMPTY, "-c", "1"], "tests/expected/empty.txt.c1.out")
}

#[test]
fn empty_c2() -> TestResult {
    run(&[EMPTY, "-c", "2"], "tests/expected/empty.txt.c2.out")
}

#[test]
fn one_c1() -> TestResult {
    run(&[ONE, "-c", "1"], "tests/expected/one.txt.c1.out")
}

#[test]
fn one_c2() -> TestResult {
    run(&[ONE, "-c", "2"], "tests/expected/one.txt.c2.out")
}

#[test]
fn two_c1() -> TestResult {
    run(&[TWO, "-c", "1"], "tests/expected/two.txt.c1.out")
}

#[test]
fn two_c2() -> TestResult {
    run(&[TWO, "-c", "2"], "tests/expected/two.txt.c2.out")
}

#[test]
fn three_c1() -> TestResult {
    run(&[THREE, "-c", "1"], "tests/expected/three.txt.c1.out")
}

#[test]
fn three_c2() -> TestResult {
    run(&[THREE, "-c", "2"], "tests/expected/three.txt.c2.out")
}

#[test]
fn ten_c1() -> TestResult {
    run(&[TEN, "-c", "1"], "tests/expected/ten.txt.c1.out")
}

#[test]
fn ten_c2() -> TestResult {
    run(&[TEN, "-c", "2"], "tests/expected/ten.txt.c2.out")
}
