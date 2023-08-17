use std::env;
use std::path::PathBuf;
use std::process::Command;

pub struct Codefile<'a> {
    pub name: &'a str,
    ending: &'a str,
    dir: PathBuf,
    command: Vec<&'a str>,
    compiled: bool,
    runtime_args: Vec<&'a str>,
}

#[derive(Debug)]
pub enum CodeFileError {
    EndingNotSupported,
    FileNotFound,
    DirectoryNotFound(std::io::Error),
}

impl From<std::io::Error> for CodeFileError {
    fn from(value: std::io::Error) -> Self {
        Self::DirectoryNotFound(value)
    }
}

pub fn get_file_ending(file_name: &str) -> Option<&str> {
    let split_file_name: Vec<&str> = file_name.split('.').collect();
    let ending = split_file_name.last()?;
    Some(ending)
}

impl<'a> TryFrom<&'a [String]> for Codefile<'a> {
    type Error = CodeFileError;

    fn try_from(args: &'a [String]) -> Result<Self, Self::Error> {

        let name = args.get(1).ok_or(Self::Error::FileNotFound)?;

        let ending = get_file_ending(name).ok_or(Self::Error::FileNotFound)?;

        let dir = env::current_dir()?;

        let mut runtime_args: Vec<&str> = Vec::new();

        // (Flag for whether or not code is compiled, full command as Vec<&str>) Ex: (true, ["python3", "main.py"])
        let (compiled, mut command) = match ending {
            "py" => {
                if env::consts::OS == "windows" {
                    (false, vec![("python")])
                } else {
                    (false, vec![("python3")])
                }
            }
            "java" => {
                
                (true, vec!["javac"])
            },
            "rs" => (true, vec!["rustc", "-o", "output"]),
            "c" => (true, vec!["gcc", "-o", "output"]),
            "cpp" => (true, vec!["g++", "-o", "output"]),
            "js" => (false, vec!["node"]),
            _ => return Err(Self::Error::EndingNotSupported),
            
        };
        command.push(name);
        if args.len() >= 2 {
            for arg in &args[2..] {
                runtime_args.push(arg)
            }
        }
        Ok(Self {
            name,
            ending,
            dir,
            command,
            compiled,
            runtime_args,
        })
    }
}
impl<'a> Codefile<'a> {
    pub fn execute(self) -> Result<(), std::io::Error>{
        let mut primary_command = Command::new(self.command[0]);
        primary_command.args(&self.command[1..]).current_dir(&self.dir);

        if !self.compiled {
            primary_command.args(&self.runtime_args);
        }

        let command_status = primary_command.status()?;
        match self.compiled {
            true if self.ending == "java" && command_status.success() => {
                let _ = Command::new(String::from("java"))
                    .arg("Main")
                    .args(&self.runtime_args)
                    .current_dir(&self.dir)
                    .status()?;
            }
            true => {
                if command_status.success() {
                    let _ = Command::new(String::from("./output"))
                        .args(&self.runtime_args)
                        .current_dir(&self.dir)
                        .status()?;
                }
            },
            false => (),
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {}

