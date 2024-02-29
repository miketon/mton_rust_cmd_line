use clap::{App, Arg};
use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config{
    files: Vec<String>,
    lines: bool,
    words: bool,
    bytes: bool,
    chars: bool,
}

pub fn run(config: Config) -> MyResult<()> {
    println!("{:#?}", config);
    Ok(())
}

pub fn get_args() -> MyResult<Config> {
    let _matches = App::new("wcr")
        // -- help info --
        .version("0.1.0")
        .author("MTON <mton@aol.com>")
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
                .long("bytes")
        )
        .arg(
            Arg::with_name("chars")
                .takes_value(false)
                .help("Show character count")
                .short("m")
                .long("chars")
        )
        .arg(
            Arg::with_name("lines")
                .takes_value(false)
                .help("Show line count")
                .short("l")
                .long("lines")
        )
        .arg(
            Arg::with_name("words")
                .takes_value(false)
                .help("Show word count")
                .short("w")
                .long("words")
        )
        .get_matches();

    Ok(Config{
        files: vec!["".to_string()],
        lines: false,
        words: false,
        bytes: false,
        chars: false
    })
}
