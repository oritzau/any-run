pub mod code_file {
    use std::path::PathBuf;
    
    use std::env;
    use std::process::Command;

    pub struct Codefile {
        pub name: String,
        pub ending: String, 
        pub dir: PathBuf,
        pub command: Vec<String>,
        pub compiled: bool,
        pub target_name: String,
    } 

    pub fn get_filename_index(args: &Vec<String>) -> usize {
        let index = 1;
        if args[1].starts_with('-') {
                return index + 2;
            }
        index
    }
    
    impl Codefile {
        pub fn new(args: Vec<String>, file_name_index: usize) -> Option<Codefile> {
            if args.len() == 0 {
                return None;
            }
            let name = args.get(file_name_index)?.to_owned();
            let split_file_name: Vec<&str> = args.get(file_name_index)?.split(".").collect();
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
            
            for arg in &args[1..] {
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

