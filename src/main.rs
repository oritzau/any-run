use std::env;
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Pass in file name, eg. 'run main.c' or 'run app.js -r -s'");
    }
    
    let file_name = args.get(1).unwrap();
    let split_file_name: Vec<&str> = args.get(1).unwrap().split('.').collect();
    let file_ending: &str = split_file_name.last().unwrap();
    let dir = env::current_dir().unwrap();
    let mut requires_second_file = false;
    let mut command = match file_ending {
        "py" => Command::new("python3"),
        "java" => Command::new("java"),
        "rs" => {
            requires_second_file = true;
            Command::new("rustc")
        }
        "c" | "cpp" => {
            requires_second_file = true;
            Command::new("gcc")
        }
        "js" => Command::new("nodejs"),
        _ => panic!("File ending not found, see 
            https://github.com/oritzau/any-run/blob/master/README.md 
            for supported file types"),
    };
    command.arg(file_name);
    if args.len() > 2 {
        command.args(&args[2..]);
    }
    if requires_second_file {
        command
            .args(["-o", "output"])
            .current_dir(dir);
    } else {
        command
            .current_dir(dir);
    }
    command.status().expect("Failed to spawn command");
    if requires_second_file {
        _ = Command::new("./output")
            .status()
            .expect("Failed to spawn command");
    }
}
