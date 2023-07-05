use run::codefile::{Codefile, get_filename_index};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    assert_ne!(args.len(), 1);
    // Crate should not be run without args, "cargo run" has no meaning
    
    let index = get_filename_index(&args); // Index of filename inside vector of args

    // If creation fails, Codefile::new() returns Option::None and the program ends with no side effects
    let file: Option<Codefile> = Codefile::new(&args, index);
    if let Some(f) = file {
        if f.execute().is_err() {
            println!("Your codefile was successfully created, but ran into an issue executing");
        }
    } else {
        // Todo: give more helpful errors
        println!("Your codefile was not successfully created");
    }
}
