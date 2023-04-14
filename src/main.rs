use run::code_file::{get_filename_index, Codefile};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Pass in file name, eg. 'run main.c' or 'run app.js -foo -bar'");
    }
    let index = get_filename_index(&args);
    let file = Codefile::new(&args, index);
    if let Some(f) = file {
        f.spawn()
    } else {
        println!("Something went wrong");
    }
}
