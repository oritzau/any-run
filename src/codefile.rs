use std::env;
use std::path::PathBuf;
use std::process::Command;

pub struct Codefile<'a> {
    pub name: &'a str,
    pub ending: &'a str,
    pub dir: PathBuf,
    pub command: Vec<&'a str>,
    pub compiled: bool,
    pub target_name: &'a str,
}

pub fn get_filename_index(args: &[String]) -> usize {
    if args.len() == 1 { return 0 }
    let index = 1;
    if args[1].starts_with('-') {
        return index + 2;
    }
    index
}

pub fn get_file_ending(file_name: &str) -> Option<&str> {
    let split_file_name: Vec<&str> = file_name.split('.').collect();
    let ending = split_file_name.last()?;
    Some(ending)
}

impl<'a> Codefile<'a> {
    pub fn new(args: &'a [String], file_name_index: usize) -> Option<Codefile<'a>> {
        if args.len() == 1 {
            return None;
        }

        // File name
        let name = args.get(file_name_index)?;

        // File ending, Ex: "js" "c" "py"
        let ending = get_file_ending(name)?;

        // Directory of target file
        let dir = env::current_dir().ok()?;

        // (Flag for whether or not code is compiled, full command as Vec<&str>) Ex: (true, ["python3", "main.py"])
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

        // Desired name of target file
        let target_name: &str = if args[1] == "-o" { &args[2] } else { "output" };

        // Adding additional flags (if any are present) to command
        for arg in &args[1..file_name_index] {
            command.push(arg);
        }
        if compiled && command.len() == 1 {
            command.push("-o");
            command.push(target_name)
        }
        for arg in &args[file_name_index..] {
            command.push(arg)
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

    pub fn execute(self) -> Result<(), std::io::Error>{
        let _ = Command::new(self.command[0])
            .args(&self.command[1..])
            .current_dir(&self.dir)
            .status()?;
        if self.compiled {
            let _ = Command::new(format!("./{}", self.target_name))
                .current_dir(&self.dir)
                .status()?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn file_name_works() {
        let vec = vec!["run".to_string(), "main.py".to_string()];
        let file = Codefile::new(&vec, 1);
        assert_eq!(file.unwrap().name, String::from("main.py"));
    }

    #[test]
    fn file_ending_works() {
        let vec = vec!["run".to_string(), "main.foo.bar.c".to_string()];
        let file = Codefile::new(&vec, 1);
        assert_eq!(file.unwrap().ending, String::from("c"));
    }

    #[test]
    fn command_works_cross_platform() {
        let vec = vec!["run".to_string(), "main.py".to_string()];
        let file = Codefile::new(&vec, 1);
        match std::env::consts::OS {
            "linux" | "macos" => assert_eq!(
                file.unwrap().command,
                vec![String::from("python3"), String::from("main.py")]
            ),
            "windows" => assert_eq!(
                file.unwrap().command,
                vec![String::from("python"), String::from("main.py")]
            ),
            _ => panic!("Invalid OS detected"),
        }
    }

    #[test]
    fn file_renamed_with_arg() {
        let vec = vec![
            "run".to_string(),
            "-o".to_string(),
            "foobar".to_string(),
            "main.c".to_string(),
        ];
        assert_eq!(
            Codefile::new(&vec, 3).unwrap().target_name,
            "foobar".to_string()
        );
    }

    #[test]
    fn get_file_ending_works() {
        let vec = vec![
            "run".to_string(),
            "-o".to_string(),
            "foobar".to_string(),
            "main.c".to_string(),
        ];
        assert_eq!(get_filename_index(&vec), 3);
    }

    #[test]
    #[should_panic]
    fn panics_with_bad_flag() {
        let vec = vec!["run".to_string(), "-o".to_string(), "main.c".to_string()];
        let file = Codefile::new(&vec, 3);
        file.unwrap();
    }

    #[test]
    fn returns_none_with_bad_flag() {
        let vec = vec!["run".to_string(), "-o".to_string(), "main.c".to_string()];
        let file = Codefile::new(&vec, 3);
        assert!(file.is_none());
    }

    #[test]
    #[should_panic]
    fn panics_with_bad_args() {
        let _file = Codefile::new(&Vec::new(), 0).unwrap();
    }

    #[test]
    #[should_panic]
    fn panics_with_bad_filetype() {
        let vec: Vec<String> = vec!["run", "my_file.txt"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let _ = Codefile::new(&vec, get_filename_index(&vec));
    }
}

