use clap::{
    App,
    Arg
};
use std::error::Error;

//---------------------------------------------------------------------------80

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config{
    files: Vec<String>,
    lines: usize,
    bytes: Option<usize>,
}

pub fn get_args() -> MyResult<Config> {

    let matches = App::new("headr")
        // --help info
        .version("0.1.0")
        .author("MTON <mton@aol.com>")
        .about("Rust head")
        // -- positional arguments
        .arg(
            Arg::with_name("files")
                .required(true)
                .help("Input file(s)")
                .default_value("-")
                .multiple(true)
                .value_name("FILE")
        )
        // -- optional arguments
        //  - unlike flags, takes_value == true
        .arg(
            Arg::with_name("bytes")
                .takes_value(true)
                .help("Number of bytes")
                // @audit : minimal to toggle as optional arg
                // - short, long ... anything else ???
                .short("c")
                .long("bytes")
                // long desc <> defaults to with_name
                // - unless we specify value_name here
                .value_name("BYTES")
        )
        .arg(
            Arg::with_name("lines")
                .takes_value(true)
                .help("Number of lines")
                .default_value("10")
                .short("n")
                .long("lines")
                .value_name("Lines")
        )
        // -- flags 
        .get_matches();

    Ok(Config {
        files: vec!["-".to_string()],
        lines: 10,
        bytes: None
    })
}

pub fn run(config: Config) -> MyResult<()> {
    println!("{:#?}", config);
    Ok(())
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
