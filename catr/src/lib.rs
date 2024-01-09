use clap::{App, Arg};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

// reduce verbosity of returning type and heap error address
type MyResult<T> = Result<T, Box<dyn Error>>;

// default all var and funcs are private
// - using 'pub' here to grant main.rs visibility
pub fn run(config: Config) -> MyResult<()> {
    for filename in &config.files{
        // borrowing &filename
        match open(&filename) {
            // good form to eprint error to stderr
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            Ok(file) => {
                // _ place holder, currently we aren't printing line numbers
                //for (_, line) in file.lines().enumerate() {
                for line in file.lines(){
                    println!("{}", line?);
                }
            }
        }
    }
    dbg!(config);
    Ok(())
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("catr")
        .version("0.1.0")
        .author("MTON <mton@aol.com>")
        .about("Rust cat")
        // positional arguments
        .arg(
            Arg::with_name("files")
                .value_name("FILE")
                .help("Input file(s)")
                // @audit : explain these entries
                .multiple(true)
                .default_value("-")
        )
        // optional arguments
        // - can have short and long names
        .arg(
            Arg::with_name("line_numbers")
                .short("n")
                .long("number")
                .help("Number lines")
                // this is a flag and does NOT take a value
                .takes_value(false) 
                // can not occur in conjunction with -b
                // - could this have been specified in the -b match instead?
                .conflicts_with("line_numbers_non_blank")
        )
        .arg(
            Arg::with_name("line_numbers_non_blank")
                .short("b")
                .long("number-nonblank")
                .help("Number lines (includes non blanks)")                
                // this is a flag and does NOT take a value
                .takes_value(false)
        )
        .get_matches();
    
    Ok(Config{
        // because there's a default value, it should be save to call unwrap()
        files: matches.values_of_lossy("files").unwrap(),
        number_lines: matches.is_present("line_numbers"),
        number_nonblank_lines: matches.is_present("line_numbers_non_blank")
    })
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        // when filename is "-"
        // - read from stdin()
        "-" => Ok(
                    Box::new(
                        BufReader::new(io::stdin())
                    )
                ),
        // else try to open given filename
        _ => Ok(
                Box::new(
                    BufReader::new(
                        File::open(filename)?
                    )
                )
            ),
    }
}
