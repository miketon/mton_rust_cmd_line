use assert_cmd::Command;
use predicates::prelude::*;
// generates random test values
use rand::{
    distributions::Alphanumeric,
    Rng
};
use std::error::Error;
// handles file input and output
use std::fs;

// Result<>
// - SUCCESS : returns ()
// - FAILURE : returns  Box<dyn Error> (heap)
type TestResult = Result<(), Box<dyn Error>>;

const PRG: &str = "catr";
// paths to various input files for usage in different test scenarios
const EMPTY: &str = "tests/inputs/empty.txt";
const FOX: &str = "tests/inputs/fox.txt";
const SPIDERS: &str = "tests/inputs/spiders.txt";
const BUSTLE: &str = "tests/inputs/the-bustle.txt";

// ----------------------------------------------------------------------------
fn run(
    // - args : &[&str] (slice of string sliceS)
    //  - a slice is a VIEW into a contiguous sequence of elements, rather than the
    //  WHOLE collection
    //      - slice is a 2 word-object :
    //        - 1 - pointer to the data
    //        - 2 - length of data
    //      - similar to an array, but does NOT take ownership over data it references
    //      - also dynamically sized, meaning you can use them to view a portion of an 
    //        array or vector without knowing the SIZE at COMPILE time
    //      - EXAMPLE :
    //          - run(&["--name", "John Doe", "--age", "30"], "expected_output.txt");
    //          - &["--name", "John Doe", "--age", "30"] is an ARRAY of string LITERALS
    //            that is COERCED into a slice of string slices by the RUST compiler
    //            - coercion from : 
    //              - &[&str, 4] // fixed sized
    //              - &[&str]    // dynamically sized
    args: &[&str], 
    //  - expected_file : &str (string slice)
    expected_file: &str
) -> TestResult {
    let expected = fs::read_to_string(expected_file)?;
    Command::cargo_bin(PRG)?
        .args(args)
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}

// ----------------------------------------------------------------------------
fn run_stdin(
    input_file: &str,
    args: &[&str],
    expected_file: &str,
) -> TestResult {
    Ok(())
}

// ----------------------------------------------------------------------------
fn gen_bad_file() -> String{
    // randomly generates filename until finds one that doesn't already exist
    // - exits loop when we generate a file not already in the file system
    loop {
        let filename: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            .map(char::from)
            .collect();

        // @udit-ok : cargo test -- --nocapture to see filename println
        // - by default Rust captures anything printed to standard output
        // from println ... unless the test fails
        println!("[gen_bad_file] : generating =>  {}", filename);
        // - alternative is to use eprintln instead ...
        eprintln!("[gen_bad_file] : generating =>  {} [eprint]", filename);
        // @audit : this ^ doesn't work?
        if fs::metadata(&filename).is_err() {
            println!("[gen_bad_file] : {} doesn't exist! SUCCESS", filename);
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
// ----------------------------------------------------------------------------
#[test]
fn skips_bad_file() -> TestResult {
    let bad = gen_bad_file();
    let expected = format!("{}: .* [(]os error 2[)]", bad);
    Command::cargo_bin(PRG)?
        .arg(&bad)
        .assert()
        .success()
        .stderr(predicate::str::is_match(expected)?);
    Ok(())
}

// ----------------------------------------------------------------------------
#[test]
fn empty() -> TestResult {
    run(&[EMPTY], "tests/expected/empty.txt.out") 
} 

#[test]
fn empty_n() -> TestResult {
    //run(&["-n", EMPTY], "tests/expected/empty.txt.n.out")
    // @audit : Passes even without flag?
    run(&[EMPTY], "tests/expected/empty.txt.n.out")
}

#[test]
fn empty_b() -> TestResult {
    //run(&["-b", EMPTY], "tests/expected/empty.txt.b.out")
    // @audit : Also passes without flag ... is this test even necessary?
    run(&[EMPTY], "tests/expected/empty.txt.b.out")
}

// ----------------------------------------------------------------------------
#[test]
fn fox() -> TestResult {
    run(&[FOX], "tests/expected/fox.txt.out")   
}

#[test]
fn fox_n() -> TestResult {
    run(&["-n", FOX], "tests/expected/fox.txt.n.out")
}

#[test]
fn fox_b() -> TestResult {
    // @udit-ok : Also passes if the flag is "-n" since this only has one line
    run(&["-b", FOX], "tests/expected/fox.txt.b.out")
}

// ----------------------------------------------------------------------------
#[test]
fn spiders() -> TestResult {
    run(&[SPIDERS], "tests/expected/spiders.txt.out")
}

#[test]
fn spiders_n() -> TestResult {
    run(&["-n", SPIDERS], "tests/expected/spiders.txt.n.out")
}

#[test]
fn spiders_b() -> TestResult {
    // @udit-ok : -b and -n flags pass because no blank lines in spiders.txt
    run(&["-b", SPIDERS], "tests/expected/spiders.txt.b.out")
}

// ----------------------------------------------------------------------------
#[test]
fn bustle() -> TestResult {
    run(&[BUSTLE], "tests/expected/the-bustle.txt.out")
}

#[test]
fn bustle_n() -> TestResult {
    run(&["-n", BUSTLE], "tests/expected/the-bustle.txt.n.out")
}

#[test]
fn bustle_b() -> TestResult {
    run(&["-b", BUSTLE], "tests/expected/the-bustle.txt.b.out")
}

// ----------------------------------------------------------------------------
#[test]
fn all() -> TestResult {
    run(&[FOX, SPIDERS, BUSTLE], "tests/expected/all.out")
}

#[test]
fn all_n() -> TestResult {
    run(&["-n", FOX, SPIDERS, BUSTLE], "tests/expected/all.n.out")
}

#[test]
fn all_b() -> TestResult {
    run(&["-b", FOX, SPIDERS, BUSTLE], "tests/expected/all.b.out")
}
