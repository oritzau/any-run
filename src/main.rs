use std::env;
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Pass in exactly one file name");
    }
    let file_name = args.get(1).unwrap();
    let split_file_name: Vec<&str> = args.get(1).unwrap().split('.').collect();
    let file_ending: &str = split_file_name.get(1).unwrap();
    let dir = env::current_dir().unwrap();
    let mut is_c_like = false;
    let mut command = match file_ending {
        "py" => Command::new("python3"),
        "java" => Command::new("java"), 
        "c" | "cpp" => {
            is_c_like = true;
            Command::new("g++")
        }
        "js" => Command::new("nodejs"),
        _ => panic!("Ending not found"),
    };
    if is_c_like {
        command.arg(file_name)
            .arg("-o")
            .arg("output")
            .current_dir(&dir)
            .status()
            .expect("Failed to spawn command");
        _ = Command::new("./output")
            .current_dir(&dir)
            .status()
            .expect("Failed to spawn command");
    } else {
        command.arg(file_name)
            .current_dir(&dir)
            .status()
            .expect("Failed to spawn command");
    }
}
