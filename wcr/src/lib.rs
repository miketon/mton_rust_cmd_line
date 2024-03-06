use clap::{
    App, 
    Arg,
    // get info from Cargo.toml
    crate_version,
    crate_authors,
};
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
    // @todo : process files wrt arguments and business logic here
    println!("{:#?}", config);
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
    if[words, bytes, chars, lines].iter().all(|v| v == &false) {
        // if all(closure==true) then execute this block
        lines = true;
        words = true;
        bytes = true;
    }

    Ok(Config{
        // using struct field initialization shorthand to set values
        files,
        lines,
        words,
        bytes,
        chars, 
    })
}
