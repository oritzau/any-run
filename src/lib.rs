use std::path::PathBuf;

use std::env;
use std::process::Command;

pub struct Codefile {
    name: String,
    ending: String,
    dir: PathBuf,
    command: Command,
    compiled: bool,
    target_name: Option<String>,
}

impl Codefile {
    fn new(args: Vec<String>) -> Codefile {
        let file_name = args.get(1).unwrap();
        let split_file_name: Vec<&str> = args.get(1).unwrap().split(".").collect();
        let file_ending: &str = split_file_name.last().unwrap();
        let dir = env::current_dir().unwrap();
        let (compiled, command) = match file_ending {
            "py" => {
            if env::consts::OS == "windows" {
                (false, Command::new("python"))
            } else {
                (false, Command::new("python3"))
            }
            },
            "java" => (false, Command::new("java")),
            "rs" => (true, Command::new("rustc")),
            "c" => (true, Command::new("g++")),
            "cpp" => (true, Command::new("g++")),
            "js" => (false, Command::new("node")),
            _ => panic!(
                "File ending not supported, see 
                https://github.com/oritzau/any-run/blob/master/README.md 
                for supported file types"
            ),
        };

        let target_name: Option<String> = if compiled {
            Some("output".to_string())
        } else {
            None
        };
        Codefile {
            name: file_name.to_owned(),
            ending: file_ending.to_string(),
            dir,
            command,
            compiled,
            target_name,
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn file_name_works() {
        let file = Codefile::new(vec!["run".to_string(), "main.py".to_string()]);
        assert_eq!(file.name, String::from("main.py"));
    }

    #[test]
    fn file_ending_works() {
        let file = Codefile::new(vec!["run".to_string(), "main.foo.bar.c".to_string()]);
        assert_eq!(file.ending, String::from("c"));
    }
}
