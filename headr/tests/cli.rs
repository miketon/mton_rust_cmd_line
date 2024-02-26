use assert_cmd::Command;
use std::error::Error;
use std::fs::{self, File};
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

fn run_stdin(
    args: &[&str],
    input_file: &str,
    expected_file: &str,
) -> TestResult {
    let mut file = File::open(expected_file)?;
    let mut buffer = Vec::new();
    // @udit-ok : why are we reading bytes ... instead of straight up strings?
    // ANSWER : because byte-for-byte comparison bypasses any encoding issues
    // - bytes are a fixed set of bools with 0 or 1 as vocabs, 
    // - opposed to text with it's large char set and NP encoding xform
    // @udit-ok : Enumerate some example applications
    // ANSWER : checksum, version control, sync/transfer ..etc
    // - basically NOT text, because of encoding from :  os, nlp diff ..etc
    
    // (A) PHOTOSHOP EQUIVALENT :
    // - binary .psd files have complex layers that encode various color modes,
    // blending options and transparency effects
    // - reading this file into a buffer is essentially FLATTENING these layers,
    // so that we can do pixel-for-pixel compare without encode or xforms
    file.read_to_end(&mut buffer)?;
    // Try get string from bytes as UTF-8 encoded text
    // - lossy == invalid text fallback to Unicode unk replacement char
    // (B) PHOTOSHOP EQUIVALENT :
    // Converting to a format suitable for web, like JPEG
    // - out of gamut colors replaced with nearest in gamut color = Unicode unk
    let expected = String::from_utf8_lossy(&buffer);
   
    // input file is assumed as UTF-8 text 
    // ? throws error if :
    // - invalid char
    // - file read fails
    let input = fs::read_to_string(input_file)?;
   
    Command::cargo_bin(PRG)?
        // @udit-ok : Explain this difference vs run()
        // ANSWER : passes content of 'input' as standard input to the command
        // @audit : Explain exactly what write_stdin is doing
        .write_stdin(input)
        .args(args)
        // creates Assert object that verifies command output where :
        .assert()
        // (1) run SUCCESS : commands exit with a status of 0
        .success()
        // (2) output SUCCESS : byte-for-byte comparison with 'expected' read 
        // (B) PHOTOSHOP EQUIVALENT :
        // "Difference" blend between two images == byte-for-byte compare
        // - results in 100% black image patch if NO DIFFERENCE
        .stdout(predicate::eq(&expected.as_bytes() as &[u8]));
    
    // returns OK indicating that test PASSED
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
fn one_stdin() -> TestResult {
    run_stdin(&[], ONE, "tests/expected/one.txt.out")
}

#[test]
fn two() -> TestResult {
    run(&[TWO], "tests/expected/two.txt.out")
}

#[test]
fn two_stdin() -> TestResult {
    run_stdin(&[], TWO, "tests/expected/two.txt.out")
}

#[test]
fn three() -> TestResult {
    run(&[THREE], "tests/expected/three.txt.out")
}

#[test]
fn three_stdin() -> TestResult {
    run_stdin(&[], THREE, "tests/expected/three.txt.out")
}

#[test]
fn ten() -> TestResult {
    run(&[TEN], "tests/expected/ten.txt.out")
}

#[test]
fn ten_stdin() -> TestResult {
    run_stdin(&[], TEN, "tests/expected/ten.txt.out")
}

//---------------------------------------------------------------------------80

#[test]
fn empty_n2() -> TestResult {
    run(&[EMPTY, "-n", "2"], "tests/expected/empty.txt.n2.out")
}

#[test]
fn empty_n4() -> TestResult {
    run(&[EMPTY, "-n", "4"], "tests/expected/empty.txt.n4.out")
}

#[test]
fn one_n2() -> TestResult {
    run(&[ONE, "-n", "2"], "tests/expected/one.txt.n2.out")
}

#[test]
fn one_n2_stdin() -> TestResult {
    run_stdin(&["-n", "2"], ONE, "tests/expected/one.txt.n2.out")
}

#[test]
fn one_n4() -> TestResult {
    run(&[ONE, "-n", "4"], "tests/expected/one.txt.n4.out")
}

#[test]
fn one_n4_stdin() -> TestResult {
    run_stdin(&["-n", "4"], ONE, "tests/expected/one.txt.n4.out")
}

#[test]
fn two_n2() -> TestResult {
    run(&[TWO, "-n", "2"], "tests/expected/two.txt.n2.out")
}

#[test]
fn two_n2_stdin() -> TestResult {
    run_stdin(&["-n", "2"], TWO, "tests/expected/two.txt.n2.out")
}

#[test]
fn two_n4() -> TestResult {
    run(&[TWO, "-n", "4"], "tests/expected/two.txt.n4.out")
}

#[test]
fn two_n4_stdin() -> TestResult {
    run_stdin(&["-n", "4"], TWO, "tests/expected/two.txt.n4.out")
}

#[test]
fn three_n2() -> TestResult {
    run(&[THREE, "-n", "2"], "tests/expected/three.txt.n2.out")
}

#[test]
fn three_n2_stdin() -> TestResult {
    run_stdin(&["-n", "2"], THREE, "tests/expected/three.txt.n2.out")
}

#[test]
fn three_n4() -> TestResult {
    run(&[THREE, "-n", "4"], "tests/expected/three.txt.n4.out")
}

#[test]
fn three_n4_stdin() -> TestResult {
    run_stdin(&["-n", "4"], THREE, "tests/expected/three.txt.n4.out")
}

#[test]
fn ten_n2() -> TestResult {
    run(&[TEN, "-n", "2"], "tests/expected/ten.txt.n2.out")
}

#[test]
fn ten_n2_stdin() -> TestResult {
    run_stdin(&["-n", "2"], TEN, "tests/expected/ten.txt.n2.out")
}

#[test]
fn ten_n4() -> TestResult {
    run(&[TEN, "-n", "4"], "tests/expected/ten.txt.n4.out")
}

#[test]
fn ten_n4_stdin() -> TestResult {
    run_stdin(&["-n", "4"], TEN, "tests/expected/ten.txt.n4.out")
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
fn empty_c4() -> TestResult {
    run(&[EMPTY, "-c", "4"], "tests/expected/empty.txt.c4.out")
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
fn one_c4() -> TestResult {
    run(&[ONE, "-c", "4"], "tests/expected/one.txt.c4.out")
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
fn two_c4() -> TestResult {
    run(&[TWO, "-c", "4"], "tests/expected/two.txt.c4.out")
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
fn three_c4() -> TestResult {
    run(&[THREE, "-c", "4"], "tests/expected/three.txt.c4.out")
}

#[test]
fn ten_c1() -> TestResult {
    run(&[TEN, "-c", "1"], "tests/expected/ten.txt.c1.out")
}

#[test]
fn ten_c2() -> TestResult {
    run(&[TEN, "-c", "2"], "tests/expected/ten.txt.c2.out")
}

#[test]
fn ten_c4() -> TestResult {
    run(&[TEN, "-c", "4"], "tests/expected/ten.txt.c4.out")
}

//---------------------------------------------------------------------------80

#[test]
fn multiple_files() -> TestResult {
    run(&[EMPTY, ONE, TWO, THREE, TEN], "tests/expected/all.out")
}

#[test]
fn multiple_files_n2() -> TestResult {
    run(&[EMPTY, ONE, TWO, THREE, TEN, "-n", "2"], "tests/expected/all.n2.out")
}

#[test]
fn multiple_files_n4() -> TestResult {
    run(&[EMPTY, ONE, TWO, THREE, TEN, "-n", "4"], "tests/expected/all.n4.out")
}

#[test]
fn multiple_files_c1() -> TestResult {
    run(&[EMPTY, ONE, TWO, THREE, TEN, "-c", "1"], "tests/expected/all.c1.out")
}

#[test]
fn multipile_files_c2() -> TestResult {
    run(&[EMPTY, ONE, TWO, THREE, TEN, "-c", "2"], "tests/expected/all.c2.out")
}

#[test]
fn multiple_files_c4() -> TestResult {
    run(&[EMPTY, ONE, TWO, THREE, TEN, "-c", "4"], "tests/expected/all.c4.out")
}
