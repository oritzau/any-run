pub mod code_file {
    use std::path::PathBuf;
    
    use std::env;
    use std::process::Command;

    pub struct Codefile {
        name: String,
        ending: String, // Currently implemented for debugging purposes, will probably deprecate
        dir: PathBuf,
        command: Vec<String>,
        compiled: bool,
        target_name: String,
    } 

    pub fn get_filename_index(args: &Vec<String>) -> usize {
        let mut index = 0;
        for arg in &args[1..] {
            match arg.as_str().chars().as_str() {
                "-" => index += 1,
                _ => break
            }
        }
        index
    }
    
    impl Codefile {
        pub fn new(args: Vec<String>, file_name_index: usize) -> Option<Codefile> {
            if args.len() == 0 {
                return None;
            }
            let name = args.get(1)?.to_owned();
            let split_file_name: Vec<&str> = args.get(1)?.split(".").collect();
            let ending: String = split_file_name.last()?.to_string();
            let dir = env::current_dir().ok()?;            
            let (compiled, mut command) = match ending.as_str() {
                "py" => {
                if env::consts::OS == "windows" {
                    (false, vec![String::from("python")])
                } else {
                    (false, vec![String::from("python3")])
                }
                },
                "java" => (false, vec![String::from("java")]),
                "rs" => (true, vec![String::from("rustc")]),
                "c" => (true, vec![String::from("gcc")]),
                "cpp" => (true, vec![String::from("g++")]),
                "js" => (false, vec![String::from("node")]),
                _ => panic!(
                    "File ending not supported, see 
                    https://github.com/oritzau/any-run/blob/master/README.md 
                    for supported file types"
                ),
            };
            
            for arg in &args[1..file_name_index] {
                command.push(arg.to_owned());
            }
            let target_name: String = if args[1].as_str() == "-o" {
                args[2].clone()
            } else {
                String::new()
            };
                    
            Some(Codefile {
                name,
                ending,
                dir,
                command,
                compiled,
                target_name,
            })
        }
    
        pub fn spawn(self) {
            let _ = Command::new(&self.command[0])
                .args(&self.command[1..])
                .current_dir(self.dir)
                .status()
                .expect("Failed to spawn command");
            if self.compiled {
                let _ = Command::new(self.target_name)
                    .status()
                    .expect("Failed to spawn secondary command");
            }
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
    
    #[test]
    fn command_works_cross_platform() {
        let file = Codefile::new(vec!["run".to_string(), "main.py".to_string()]);
        match std::env::consts::OS {
            "linux" | "macos" => assert_eq!(file.command, vec![
                String::from("python3"),
                String::from("main.py")
            ]),
            "windows" => assert_eq!(file.command, vec![
                String::from("python"),
                String::from("main.py")
            ]),
            _ => panic!("Invalid OS detected"),
        }
    }

    #[test]
    fn command_works_with_args() {
        let file = Codefile::new(vec![
            "run".to_string(), 
            "main.c".to_string(), 
            "-r".to_string(), 
            "-foo".to_string()
        ]);
        assert_eq!(file.command, vec![
            "gcc".to_string(),
            "main.c".to_string(),
            "-r".to_string(),
            "-foo".to_string(),
        ])
    }

    #[test]
    #[should_panic]
    fn panics_with_bad_args() {
        let _file = Codefile::new(Vec::new());
    }

}
