use clap::{
    App,
    Arg
};
use std::fs::File;
use std::io;
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
                // by convention the "-" char signals stdin to bash tools
                .default_value("-")
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
                // mutually exclusive with lines : one or the other only
                .conflicts_with("lines")
                // @udit-ok : why is this needed ... all tests pass ...
                // and there is no default value ...
                // - is it because we handle it as Some() or None on return?
                // ANSWER : Some() is handled at parsing as opposed to init
                // - good form to be explicit that this takes a value ...
                // - also all tests passing is not an indicator of idiomatic code
                // ALSO : takes_value==true indicates this is NOT a FLAG
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

    // alternative is `unwrap_or_else(lambda)`
    // - this makes sense when the lambda is computationally expensive
    // - because, it runs LAZILY and are only called when unwrap fails
    // unwrap_or_default is EAGER would add an op 
    // - in this case the default is a static string value : "files"
    // - so neglible computation overhead
    let files = matches.values_of_lossy("files").unwrap_or_default();
   
    let lines = matches
        .value_of("lines")
        .map(parse_positive_int)
        .transpose()
        .map_err(|e| format!("illegal line count -- {}", e))?;

    let bytes = matches
        // returns Option<&str>
        .value_of("bytes")
        // -1- ::map unpacks &str from Some and sends it to parse
        // -2-  parse returns Option<Result<T, E>> 
        //  - where T is usize 
        .map(parse_positive_int)
        // transpose between Option<Result<T, E>> => Result<Option<T>, E> 
        .transpose()
        // error is deferred until after transpose
        // and transforms the error variant of the Result
        .map_err(|e| format!("illegal byte count -- {}", e))
        // handles any error by immediately returning it
        ?;
       
//---------------------------------------------------------------------------80

    Ok(Config {
        files,
        // @udit-ok : Why do we have to unwrap here?
        // ANSWER : Ah because config defined this as a usize
        // ... and we have returned it as an Option<usize> so we need to unwrap
        lines : lines.unwrap(), // has default arg and safe to unwrap
        // leave this as an Option, matches config define
        bytes,    
    })
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        // take input from stdin
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        // else try to read from file
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
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
