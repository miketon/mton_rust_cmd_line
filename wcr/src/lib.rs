use clap::{
    // get info from Cargo.toml
    crate_authors,
    crate_version,
    App,
    Arg,
};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: bool,
    words: bool,
    bytes: bool,
    chars: bool,
}

#[derive(Debug, PartialEq)]
pub struct FileInfo {
    num_lines: usize,
    num_words: usize,
    num_bytes: usize,
    num_chars: usize,
}

pub fn run(config: Config) -> MyResult<()> {
    // @todo : process files wrt arguments and business logic here
    let mut lines_total = 0;
    let mut words_total = 0;
    let mut bytes_total = 0;
    let mut chars_total = 0;

    for filename in &config.files {
        match open(filename) {
            Err(err) => eprintln!("[{}]: --> [error] {}", filename, err),
            Ok(file) => {
                if let Ok(info) = count(file) {
                    // display count from current file
                    println!(
                        "{}{}{}{}{}",
                        format_field(info.num_lines, config.lines),
                        format_field(info.num_words, config.words),
                        format_field(info.num_bytes, config.bytes),
                        format_field(info.num_chars, config.chars),
                        if filename == "-" {
                            // don't print filename if stdin
                            // @audit : what's the tradeoff between :
                            // - "".to_string()
                            // - format!("")
                            // "".to_string()
                            format!("")
                        } else {
                            format!(" {}", &filename)
                        },
                    );

                    // accumulate count across files
                    lines_total += info.num_lines;
                    words_total += info.num_words;
                    bytes_total += info.num_bytes;
                    chars_total += info.num_chars;
                }
            }
        }
    }

    // print total if more than one file was processed
    if config.files.len() > 1 {
        println!(
            "{}{}{}{} total",
            format_field(lines_total, config.lines),
            format_field(words_total, config.words),
            format_field(bytes_total, config.bytes),
            format_field(chars_total, config.chars),
        );
    }

    Ok(())
}

pub fn get_args() -> MyResult<Config> {
    // [args] parsing
    let matches = App::new("wcr")
        // -- help info --
        .version(crate_version!())
        .author(crate_authors!())
        .about("Rust wc")
        // -- positional args --
        .arg(
            Arg::with_name("files")
                .value_name("FILE")
                .help("Input file(s)")
                .default_value("-") // << STD_IN pass thru convention
                .multiple(true),
        )
        // -- optional args --
        // - sorted by long("$")
        .arg(
            Arg::with_name("bytes")
                .takes_value(false) // << false == this is a FLAG
                .help("Show byte count")
                .short("c")
                .long("bytes"),
        )
        .arg(
            Arg::with_name("chars")
                .conflicts_with("bytes")
                .takes_value(false)
                .help("Show character count")
                .short("m")
                .long("chars"),
        )
        .arg(
            Arg::with_name("lines")
                .takes_value(false)
                .help("Show line count")
                .short("l")
                .long("lines"),
        )
        .arg(
            Arg::with_name("words")
                .takes_value(false)
                .help("Show word count")
                .short("w")
                .long("words"),
        )
        .get_matches();

    // [io] file(s) check
    // default -> '-' which signifies STD_IN flow
    // - so we can safely unwrap or default
    let files = matches.values_of_lossy("files").unwrap_or_default();

    // [flag] check
    let mut lines = matches.is_present("lines");
    let mut words = matches.is_present("words");
    let mut bytes = matches.is_present("bytes");
    let chars = matches.is_present("chars");

    // @udit-ok : Explain what this is doing
    // ANSWER : mimics default behaviour of Unix wc command
    // - which is if no flag set, lines, words, bytes == TRUE
    // - create temp list using slice with all flags [words, bytes, chars lines]
    // - then slice::iter() over all() and test each element where :
    //   - |v| v == &false // lambda check if each |v| is false
    // - if all() are FALSE, set lines, words and bytes to TRUE
    // @PHOTOSHOP : lines, words, bytes, chars are layers
    // - checking if all layers are hidden before proceeding with a
    // certain action :
    //   - if all() are hidden, set : lines, words, bytes => visible
    // @udit-ok : Explain why compare ref &false vs value false
    // ANSWER : because iter() yields REF to each element of the array
    // ... and not to the VALUE of each element in the array
    //  - checking v == &false directly, SKIPS having to deref v ...
    //    - but if we INSIST on dereferencing v, here are options
    //      - .all(|&v| v == false) // idiomatic deref element : a & bool
    //      - .all(|v| *v == false) // ref to references : a && bool
    // @PHOTOSHOP : Instead of the direct eye icon to toggle visibility per layer
    // ... iter() would return a `tag` for visibility per layer
    // - 'tag' is a level of indirection, like how the ref &false
    if [words, bytes, chars, lines].iter().all(|v| v == &false) {
        // -- shorter equivalent but arguably HARDER to READ
        //if[words, bytes, chars, lines].iter().all(|v| !v ) {
        // if all(closure==true) then execute this block
        lines = true;
        words = true;
        bytes = true;
    }

    Ok(Config {
        // using struct field initialization shorthand to set values
        files,
        lines,
        words,
        bytes,
        chars,
    })
}

// @udit-ok : Explain impl BufRead
// ANSWER : file can be any type that implements BufRead
// - BufReader, Cursor are compatible
pub fn count(mut file: impl BufRead) -> MyResult<FileInfo> {
    let mut num_lines = 0;
    let mut num_words = 0;
    let mut num_bytes = 0;
    let mut num_chars = 0;
    let mut line = String::new();

    // implement code to actually count here
    loop {
        let line_bytes = file.read_line(&mut line)?;
        // @audit : explain the reason on breaking loop on value 0 specifically
        if line_bytes == 0 {
            break;
        }
        num_bytes += line_bytes;
        num_lines += 1;
        num_words += line.split_whitespace().count();
        num_chars += line.chars().count();
        // @audit : explain why we are clearing the line here
        line.clear();
    }

    Ok(FileInfo {
        num_lines,
        num_words,
        num_bytes,
        num_chars,
    })
}

// @udit-ok : Explain what is happening with BufReader -> BufRead Result
// ANSWER : open returns a boxed type that implements BufRead
// - BufReader has BufRead impl
fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

fn format_field(value: usize, show: bool) -> String {
    if show {
        // @audit : explain why no ; needed
        format!("{:>8}", value)
    } else {
        "".to_string()
    }
}

// *** test modules ***
// @audit : Explain difference between inlining a test vs MOD test
#[cfg(test)] // cfg enables CONDITIONAL compilation - bin only when testing
mod tests {
    // @audit : Explain the tradeoff between
    // - use super::format_field
    // - use cargo::format_field
    use super::{count, format_field, FileInfo};
    use std::io::Cursor;

    #[test]
    fn test_count() {
        let text = "I don't want the world. I just want your half.\r\n";
        let info = count(Cursor::new(text));
        assert!(info.is_ok());
        let expected = FileInfo {
            num_lines: 1,
            num_words: 10,
            num_chars: 48,
            num_bytes: 48,
        };

        assert_eq!(info.unwrap(), expected);
    }

    #[test]
    fn test_format_field() {
        assert_eq!(format_field(1, false), "");
        #[rustfmt::skip] // line up spaces
        assert_eq!(format_field( 3, true), "       3");
        assert_eq!(format_field(10, true), "      10");
    }
}
