use clap::{App, Arg};
use std::error::Error;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            files: Vec::new(),
            number_lines: false,
            number_nonblank_lines: false,
        }
    }
}

type MyResult<T> = Result<T, Box<dyn Error>>;

// default all var and funcs are private
// - using 'pub' here to grant main.rs visibility
pub fn run(config: Config) -> MyResult<()> {
    println!("Lib.rs:: Hello World!");
    dbg!(config);
    Ok(())
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("catr")
        .version("0.1.0")
        .author("MTON <mton@aol.com>")
        .about("Rust cat")
        .arg(
            Arg::with_name("files")
                .value_name("FILE")
                .help("Input file(s) [default: -]")
        )
        .arg(
            Arg::with_name("line_numbers")
                .short("n")
                .long("number")
                .help("Number lines")
        )
        .arg(
            Arg::with_name("line_numbers_non_blank")
                .short("b")
                .long("number-nonblank")
                .help("Number lines (includes non blanks)")                
        )
        .get_matches();
    
    Ok(Config::default())
}
