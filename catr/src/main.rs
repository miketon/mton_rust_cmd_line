fn main() {
    // try to execute lib.rs run function
    // @audit : explain modules ... is src/lib.rs an implicit constant?
    // ANSWER : lib/run() is accessed through `catr` the project crate
    if let Err(e) = catr::get_args().and_then(catr::run) {
        // use error print line to print error message to STDERR
        eprintln!("{}", e);
        // Exit program with a nonzero value to indicate an error
        // @audit : Explain why this is important for command line tools
        std::process::exit(1);
    }
}
