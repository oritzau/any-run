use std::env;
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Pass in file name, eg. 'run main.c' or 'run app.js -foo -bar'");
    }
    
    let file_name = args.get(1).unwrap();
    let split_file_name: Vec<&str> = args.get(1).unwrap().split('.').collect();
    let file_ending: &str = split_file_name.last().unwrap();
    let dir = env::current_dir().unwrap("Failed to locate current directory");
    let mut requires_second_file = false;
    let mut command = match file_ending {
        "py" => Command::new("python3"),
        "java" => Command::new("java"),
        "rs" => {
            requires_second_file = true;
            Command::new("rustc")
        }
        "c" => {
            requires_second_file = true;
            Command::new("gcc")
        }
        "cpp" => {
            requires_second_file = true;
            Command::new("g++")
        }
        "js" => Command::new("node"),
        _ => panic!("File ending not supported, see 
            https://github.com/oritzau/any-run/blob/master/README.md 
            for supported file types"),
    };
    // Ex: gcc -> gcc -o output 
    if requires_second_file {
        command
            .args(["-o", "output"]) // TODO: allow for -o arg to rename secondary file
        
    // Ex: gcc -o output -> gcc -o output main.c
    command.arg(file_name);
    
    // Ex gcc -o output main.c -> gcc -o output main.c -r -foo
    if args.len() > 2 {
        command.args(&args[2..]);
    }

    // Points to current directory
    command.current_dir(dir);

    // Executes
    command.status().expect("Failed to spawn command");
    if requires_second_file {
        _ = Command::new("./output")
            .status()
            .expect("Failed to spawn command");
    }
}
