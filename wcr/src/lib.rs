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
    let mut file_count = 0;
    let mut num_lines_total = 0;
    let mut num_words_total = 0;
    let mut num_bytes_total = 0;
    let mut num_chars_total = 0;

    for filename in &config.files {
        match open(filename) {
            Err(err) => eprintln!(" -- filename -- {} : -- err -- {}", filename, err),
            Ok(file) => {
                if let Ok(info) = count(file) {
                    print_info(&info, &config, filename);
                    // update totals
                    file_count += 1;
                    num_lines_total += info.num_lines;
                    num_words_total += info.num_words;
                    num_bytes_total += info.num_bytes;
                    num_chars_total += info.num_chars;
                }
            }
        }
    }

    // print totals if more than one file was processed
    if file_count > 1 {
        let total_info = FileInfo {
            num_lines: num_lines_total,
            num_words: num_words_total,
            num_bytes: num_bytes_total,
            num_chars: num_chars_total,
        };
        print_info(&total_info, &config, "total");
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
        if line_bytes == 0 {
            break;
        }
        num_bytes += line_bytes;
        num_lines += 1;
        num_words += line.split_whitespace().count();
        num_chars += line.chars().count();
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

fn print_info(info: &FileInfo, config: &Config, filename: &str) {
    let mut outputs = Vec::new();

    if config.lines {
        outputs.push(format!("{:>8}", info.num_lines));
    }
    if config.words {
        outputs.push(format!("{:>8}", info.num_words));
    }
    if config.bytes {
        outputs.push(format!("{:>8}", info.num_bytes));
    }
    if config.chars {
        outputs.push(format!("{:>8}", info.num_chars));
    }

    // join outputs with spaces and print
    let output = outputs.join("");

    if filename != "-" {
        println!("{} {}", output, filename);
    } else {
        println!("{}", output);
    }
}

// *** test modules ***
// @audit : Explain difference between inlining a test vs MOD test
#[cfg(test)] // cfg enables CONDITIONAL compilation - bin only when testing
mod tests {
    use super::{count, FileInfo};
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
}
