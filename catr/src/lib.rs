use clap::{App, Arg};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

// == "channels" of an image
// - r: number_lines
// - g: files
//  - files is Vec<> vs bool and requires more data
//  - so we switched it to line up with the green channel 
//  because human vision is more indexed to green cones
// - b: number_nonblank_lines
#[derive(Debug)]
pub struct Config {
    number_lines: bool,
    files: Vec<String>,
    number_nonblank_lines: bool,
}

// == aliasing is a uniform way to manage "noise"
// - where noise is analagous to errors
// reduce verbosity of returning type and heap error address
type MyResult<T> = Result<T, Box<dyn Error>>;

// == akin to processing multiple image channels in a batch
// - where each file it attempts to open and process
// default all var and funcs are private
// - using 'pub' here to grant main.rs visibility
pub fn run(config: Config) -> MyResult<()> {
    // @udit-ok : why borrow?
    // ANSWER : iterating over filenames using a reference is typical
    // this avoids taking ownership unnecessarily
    // - because we are only reading and not writing to these files
    for filename in &config.files{
        // no need to borrow `filename` again since it's already referenced from
        // the iteration
        // match open(&filename) {
        // @udit-ok : the tradeoff between borrowing @iteration vs @match?
        // ANSWER : borrow @iteration is more idiomatic in Rust 
        // - more clarity that the value is being READ only, NO WRITE 
        // - good practice is to borrow at minimal scope necessary
        match open(filename) {
            // good form to eprint error to stderr
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            Ok(file) => {
                let mut valid_line_id = 0; // skip if line is empty
                // returning id and result from enumerate ~ perf diff
                // - line_id : helps clarify when we want to print ALL line 
                // number
                for (line_id, line) in file.lines().enumerate(){
                    // Unwrap once and store the value 
                    // - error because calling line? a 2nd time tries to move 
                    // a value that's no longer there
                    // @udit-ok : shadowing is more Rustic, but is it more 
                    // performant?
                    // - reusing 'line' token reduces the naming table?
                    //   - that would be bullshit tho, compiler should optimize
                    //   that out!
                    // - 'line_result' is arguably more explicit and readable
                    // ANSWER : compiler is likely to optimize out diff in perf
                    // with shadowing
                    // ... but not GUARANTEED, so it's more Rustic to manually
                    // shadow
                    let line = line?;
                    if config.number_lines{
                        // {:>6} = text aligned to right with 6 characters
                        // {:<6} = left justified
                        // {:^6} = center justified
                        println!("{:>6}\t{}", line_id+1, line);
                    }
                    else if config.number_nonblank_lines{
                        // line.is_empty() == true if length = 0, false if 
                        // there is whitespace tho
                        // line.trim().is_empty() == true if only whitespace
                        // (tabs...etc)
                        if line.is_empty(){
                            println!();
                        }
                        else{
                            valid_line_id += 1;
                            println!("{:>6}\t{}", valid_line_id, line);
                        }
                    }
                    else{
                        println!("{}", line);
                    }
                }
            }
        }
    }
    dbg!(config);
    Ok(())
}

// == initializes color depth
// - configures the options and flags of how output is rendered
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
        // because there's a default value, it should be safe to call unwrap()
        files: matches.values_of_lossy("files").unwrap(),
        number_lines: matches.is_present("line_numbers"),
        number_nonblank_lines: matches.is_present("line_numbers_non_blank")
    })
}

// == handles input source
// - either live stream (stdin) or from disc (file)
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
