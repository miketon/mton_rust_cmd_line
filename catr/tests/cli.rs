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
// validate how cat runs
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
// validate how cat handles stdin piping
fn run_stdin(
    // input file to read
    input_file: &str,
    // &[&str] - slice of args string sliceS
    args: &[&str],
    // file of expected result to match loss against
    expected_file: &str,
) -> TestResult {
    // read entire content of input and expected files into 2 String (immutable) :
    // - 1 - input
    // - 2 - expected
    // this op can fail so :
    // - '?' would return early with an error
    //   - returns Ok() or Err<Box>
    //   - can only be used in functions that return
    //     - 1 - Result
    //     - 2 - Option
    //     - 3 - Try trait
    let input = fs::read_to_string(input_file)?;
    let expected = fs::read_to_string(expected_file)?;
    Command::cargo_bin(PRG)?
        .args(args)
        // Write the contents of 'input' to the standard input (stdin) of the command
        // @udit-ok : Ah! this is why we don't need to pass the '<' pipe in args
        .write_stdin(input)
        // Execute command and obtain 'assert' object to check
        .assert()
        // Ensure command executed successfully where exit_status == 0
        .success()
        // Check that output of stdout matches text from 'expected' file 
        // - If this assertion fails :
        //   - Panic
        //   - Fail Test Run
        .stdout(expected);
    // This point indicates that ALL assertions PASS, return 'Ok(())'
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

// ----------------------------------------------------------------------------
#[test]
fn bustle_stdin() -> TestResult {
    run_stdin(
        BUSTLE, 
        &["-"], 
        "tests/expected/the-bustle.txt.stdin.out"
    )
}

#[test]
fn bustle_stdin_n() -> TestResult {
    run_stdin(
        BUSTLE,
        &["-n", "-"],
        "tests/expected/the-bustle.txt.stdin.n.out"
    )
}

#[test]
fn bustle_stdin_b() -> TestResult {
    run_stdin(
        BUSTLE,
        // @udit-ok : Explain the difference between having the '<' char and not
        // &["-b", "-", "<"],
        // ANSWER : '<' is valid in a shell context, but not a Rust context
        // - '<' is redirection in shell, and is not a valid argument
        //  - if we don't remove it, the program still runs, because it IGNORES 
        //  invalid arguments
        // - '.write_stdin(input)' is the Rust equivalent
        &["-b", "-"],
        "tests/expected/the-bustle.txt.stdin.b.out"
    )
}
