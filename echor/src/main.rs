use clap::{
    App,
    Arg
}; // import the clap::App struct

fn main() {
    let matches = App::new("echor") // create new app with the name 'echor'
        .version("0.1.0") // use semantic version information
        .author("Mike Ton <mike.ton@gmail.com>") // name and email so people know where to send
                                                 // money lol
        .about("Rust echo") // short description of the program
        .arg(
            Arg::with_name("text")
                .value_name("Text")
                .help("Input text")
                .required(true)
                .min_values(1),
        )
        .arg(
            Arg::with_name("omit_newline")
                .short("n")
                .help("Do not print newline")
                .takes_value(false),
        )
        .get_matches(); // Tell the `App` to parse the arguments

    let text = matches.values_of_lossy("text").unwrap();
    let omit_newline = matches.is_present("omit_newline");

    // if is an expression not a statement
    // - it can return a value
    // - it's more rustic than having let be a mut that we assign
    // multiple times!
    print!(
        "{}{}",
        text.join(" "),
        if omit_newline {""} else {"\n"}
    );
}
