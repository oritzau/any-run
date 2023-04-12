use std::env;
use run::{self, Codefile};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Pass in file name, eg. 'run main.c' or 'run app.js -foo -bar'");
    }
    let file = Codefile::new(args);
    file.spawn()
}
