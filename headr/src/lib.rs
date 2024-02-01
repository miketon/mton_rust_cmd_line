use clap::{
    App,
    Arg
};
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::error::Error;

//---------------------------------------------------------------------------80

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config{
    files: Vec<String>,
    lines: usize,
    bytes: Option<usize>,
}

pub fn run(config: Config) -> MyResult<()> {
    for filename in &config.files {
        match open(filename) {
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            Ok(file) => {
                for line in file.lines(){
                    let line = line?;
                    println!("{}", line);
                }
            }
        }
    }
    Ok(())
}

pub fn get_args() -> MyResult<Config> {

    let matches = App::new("headr")
        // --help info
        .version("0.1.0")
        .author("MTON <mton@aol.com>")
        .about("Rust head")
        // -- positional arguments
        .arg(
            Arg::with_name("files") // name of argument for code access
                .multiple(true)
                .default_value("-- (file path required) --")
                // @udit-ok : because we have a default value, 
                // 'required' is UNNECESSARY and CONTRADICTORY
                //.required(true)
                .help("Input file(s)")
                .value_name("FILES") // descriptive name for USAGE DOC
        )
        // -- optional arguments
        //  - unlike flags, takes_value == true
        .arg(
            Arg::with_name("lines")
                .default_value("10")
                // @udit-ok : Why is takes_values not needed here?
                // - ah is it because there's a default value?
                // ANSWER : good form to be explicit that this takes value 
                // ... even if there's a default
                .takes_value(true)
                .help("Number of lines")
                .short("n")
                .long("lines")
                .value_name("LINES")
        )
        .arg(
            Arg::with_name("bytes")
                .conflicts_with("lines")
                // @udit-ok : why is this needed ... all tests pass ...
                // and there is no default value ...
                // - is it because we handle it as Some() or None on return?
                // ANSWER : Some() is handled at parsing as opposed to init
                // - good form to be explicit that this takes a value ...
                // - also all tests passing is not an indicator of idiomatic code
                .takes_value(true)
                .help("Number of bytes")
                // @udit-ok : minimal to toggle as optional arg
                // - short, long ... anything else ???
                // ANSWER : simply don't mark it as 'required'
                .short("c")
                .long("bytes")
                // long desc <> defaults to with_name
                // - unless we specify value_name here
                .value_name("BYTES")
        ) 
        // -- flags 
        .get_matches();

//---------------------------------------------------------------------------80

    let files = matches.values_of_lossy("files").unwrap_or_default();
    
    let lines = 
    match parse_positive_int(
        // default 'lines' value is 10 so will return a valid positive number
        // ... will be PROBLEMATIC if that changes
        // @audit : BAD FORM to implicitly bury that dependency
        matches.value_of("lines").unwrap_or_default()
    ){
        Ok(num) => num,
        Err(err) => {
            eprintln!("illegal line count -- {} {}", err, matches.value_of("lines").unwrap_or_default());
            return Err(err);
        }
    };

    let bytes = match matches.value_of("bytes"){
        Some(value) => Some(
            match parse_positive_int(value){
                Ok(num) => num,
                Err(err) => {
                    eprintln!("illegal byte count -- {}", err);
                    return Err(err);
                }
            }
        ),
        // Default to None if no value provided
        None => None,
    };
 
//---------------------------------------------------------------------------80

    Ok(Config {
        files,
        lines,
        bytes,    
    })
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    Ok(
        Box::new(
            BufReader::new(
                File::open(filename)?
            )
        )
    )
}

fn parse_positive_int(val: &str) -> MyResult<usize> {
    // - 1 - parse value as a usize
    //  - parse returns Result<usize, ParseIntError> in this context
    // @udit-ok : How was this config to parse usize?
    // ANSWER : Rust infers usize from the return type ...
    // - GPT explain here please ---
    // Inference types available :
    //  - Function Return Type (this)
    //  - Type Annotations : let val: usize = "42".parse().unwrap()
    //  - Type Expectations in Operations : val.parse() * 1.0_f32
    //  - Type Bounds and Traits
    //  - Default Type Parameters
    //  - Compiler Errors and Type Checking
    match val.parse(){
        // - 2 - check if positive : greater than zero
        // if parse succeeds and value is positive, return that
        Ok(n) if n > 0 => Ok(n),
        // - 3 - Else return Err with given value
        _ => Err(val.into()),
    }
}

#[test]
fn test_parse_positive_int() {
    // 3 is an OK integer
    let res = parse_positive_int("3");
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 3);

    // Any string is an error
    let res = parse_positive_int("foo");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "foo".to_string());

    // A zero is an error
    let res = parse_positive_int("0");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "0".to_string());
}
