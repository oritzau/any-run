pub mod code_file {
    use std::path::PathBuf;

    use std::env;
    use std::process::Command;

    pub struct Codefile<'a> {
        pub name: &'a str,
        pub ending: &'a str,
        pub dir: PathBuf,
        pub command: Vec<&'a str>,
        pub compiled: bool,
        pub target_name: &'a str,
    }

    pub fn get_filename_index(args: &Vec<String>) -> usize {
        let index = 1;
        if args[1].starts_with('-') {
            return index + 2;
        }
        index
    }

    pub fn get_file_ending(file_name: &str) -> Option<&str> {
        let split_file_name: Vec<&str> = file_name.split(".").collect();
        let ending = split_file_name.last()?;
        Some(ending)
    }

    impl<'a> Codefile<'a> {
        pub fn new(args: &'a Vec<String>, file_name_index: usize) -> Option<Codefile<'a>> {
            if args.len() == 0 {
                return None;
            }
            let name = args.get(file_name_index)?;
            let ending = get_file_ending(&name)?;
            let dir = env::current_dir().ok()?;
            let (compiled, mut command) = match ending {
                "py" => {
                    if env::consts::OS == "windows" {
                        (false, vec![("python")])
                    } else {
                        (false, vec![("python3")])
                    }
                }
                "java" => (false, vec!["java"]),
                "rs" => (true, vec!["rustc"]),
                "c" => (true, vec!["gcc"]),
                "cpp" => (true, vec!["g++"]),
                "js" => (false, vec!["node"]),
                _ => panic!(
                    "File ending not supported, see 
                    https://github.com/oritzau/any-run/blob/master/README.md 
                    for supported file types"
                ),
            };
            let target_name: &str = if args[1] == "-o" { &args[2] } else { "output" };

            for arg in &args[1..file_name_index] {
                command.push(&arg);
            }

            if compiled && command.len() == 1 {
                command.push("-o");
                command.push(target_name)
            }

            for arg in &args[file_name_index..] {
                command.push(&arg)
            }

            Some(Self {
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
                let _ = Command::new(format!("./{}", self.target_name))
                    .status()
                    .expect("Failed to spawn secondary command");
            }
        }
    }
}

