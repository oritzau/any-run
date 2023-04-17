use run::code_file::{get_filename_index, Codefile};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Crate should not be run without args, "cargo run" has no meaning
    if args.len() < 2 {
        panic!("Pass in file name, eg. 'run main.c' or 'run app.js -foo -bar'");
    }
    let index = get_filename_index(&args); // Index of filename inside vector of args

    // If creation fails, Codefile::new() returns none and the program ends with no side effects
    let file: Option<Codefile> = Codefile::new(&args, index);
    if let Some(f) = file {
        f.execute()
    } else {
        // Todo: give more helpful errors
        println!("Something went wrong");
    }
}
