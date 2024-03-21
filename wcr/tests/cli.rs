use assert_cmd::Command;
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
fn empty() -> TestResult {
    run(&[EMPTY], "tests/expected/empty.txt.out")
}

// --------------------------------------------------------------------------80
#[test]
fn fox() -> TestResult {
    run(&[FOX], "tests/expected/fox.txt.out")
}

#[test]
fn fox_bytes() -> TestResult {
    run(&["--bytes", FOX], "tests/expected/fox.txt.c.out")
}

#[test]
fn fox_chars() -> TestResult {
    run(&["--chars", FOX], "tests/expected/fox.txt.m.out")
}

#[test]
fn fox_words() -> TestResult {
    run(&["--words", FOX], "tests/expected/fox.txt.w.out")
}

#[test]
fn fox_lines() -> TestResult {
    run(&["--lines", FOX], "tests/expected/fox.txt.l.out")
}

#[test]
fn fox_words_bytes() -> TestResult {
    run(&["-w", "-c", FOX], "tests/expected/fox.txt.wc.out")
}

#[test]
fn fox_words_lines() -> TestResult {
    run(&["-w", "-l", FOX], "tests/expected/fox.txt.wl.out")
}

#[test]
fn fox_bytes_lines() -> TestResult {
    run(&["-l", "-c", FOX], "tests/expected/fox.txt.cl.out")
}

// --------------------------------------------------------------------------80
#[test]
fn atlamal() -> TestResult {
    run(&[ATLAMAL], "tests/expected/atlamal.txt.out")
}

#[test]
fn atlamal_bytes() -> TestResult {
    run(&["-c", ATLAMAL], "tests/expected/atlamal.txt.c.out")
}

#[test]
fn atlamal_words() -> TestResult {
    run(&["-w", ATLAMAL], "tests/expected/atlamal.txt.w.out")
}

#[test]
fn atlamal_lines() -> TestResult {
    run(&["-l", ATLAMAL], "tests/expected/atlamal.txt.l.out")
}

#[test]
fn atlamal_words_bytes() -> TestResult {
    run(&["-w", "-c", ATLAMAL], "tests/expected/atlamal.txt.wc.out")
}

#[test]
fn atlamal_words_lines() -> TestResult {
    run(&["-w", "-l", ATLAMAL], "tests/expected/atlamal.txt.wl.out")
}

#[test]
fn atlamal_bytes_lines() -> TestResult {
    run(&["-l", "-c", ATLAMAL], "tests/expected/atlamal.txt.cl.out")
}

#[test]
fn atlama_stdin() -> TestResult {
    let input = fs::read_to_string(ATLAMAL)?;
    let expected = fs::read_to_string("tests/expected/atlamal.txt.stdin.out")?;
    Command::cargo_bin(PRG)?
        // @audit : is write_stdin the equivalent of | in bash shell?
        .write_stdin(input)
        .assert()
        .stdout(expected);
    Ok(())
}

// --------------------------------------------------------------------------80
#[test]
fn test_all() -> TestResult {
    run(&[EMPTY, FOX, ATLAMAL], "tests/expected/all.out")
}

#[test]
fn test_all_lines() -> TestResult {
    run(&["-l", EMPTY, FOX, ATLAMAL], "tests/expected/all.l.out")
}

#[test]
fn test_all_words() -> TestResult {
    run(&["-w", EMPTY, FOX, ATLAMAL], "tests/expected.all.w.out")
}

#[test]
fn test_all_bytes() -> TestResult {
    run(&["-c", EMPTY, FOX, ATLAMAL], "tests/expected/all.c.out")
}

#[test]
fn test_all_words_bytes() -> TestResult {
    run(&["-cw", EMPTY, FOX, ATLAMAL], "tests/expected/all.wc.out")
}

#[test]
fn test_all_words_lines() -> TestResult {
    run(&["-wl", EMPTY, FOX, ATLAMAL], "tests/expected/all.wl.out")
}

#[test]
fn test_all_bytes_lines() -> TestResult {
    run(&["cl", EMPTY, FOX, ATLAMAL], "tests/expected/all.cl.out")
}

// --------------------------------------------------------------------------80

#[test]
fn skips_bad_file() -> TestResult {
    let bad = gen_bad_file();
    // @udit-ok : is there a more elegant way to format this?
    // ANSWER : this is NOT much better :
    // let expected = format!(r".*{filename}.* \(os error 2\)", filename = bad);
    // - used a raw string (indicated by r"") to avoid needing to escape the
    // backslashes for the regex
    // - but since there are no backslashes in this pattern, it doesn't make a
    // difference here
    let expected = format!(".*{}.* [(]os error 2[)]", bad);
    Command::cargo_bin(PRG)?
        .arg(bad)
        .assert()
        .success()
        .stderr(predicate::str::is_match(expected)?);
    Ok(())
}

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

// test for default : lines, words and bytes
// - [x] DEFAULT ORDER : lines, words, byte/characters

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
// [x] [multi-files] => [total] # lines | words | byte
// [x] [file-error] => Nonexistent files note warning to STDERR as files process
