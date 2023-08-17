use run::codefile::{Codefile, CodeFileError};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    assert_ne!(args.len(), 1);

    let file = Codefile::try_from(&args[..]);

    match file {
        Ok(f) => {
            let file_name: String = String::from(f.name);
            if let Err(_) = f.execute() {
                println!("Codefile `{}` failed to run", file_name);
            }
        }
        Err(CodeFileError::FileNotFound) => println!("poly-run: Couldn't find file"),
        Err(CodeFileError::EndingNotSupported) => println!("poly-run: File ending not supported. See https://github.com/oritzau/poly-run/blob/master/README.md for supported filetypes"),
        Err(CodeFileError::DirectoryNotFound(_)) => println!("poly-run: Could not find directory information"),
    }
}
