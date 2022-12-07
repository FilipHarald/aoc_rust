use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug)]
pub enum TerminalIO {
    Cd(String),
    Ls,
    OutputFile { name: String, size: u32 },
    OutputDir { name: String },
}

impl FromStr for TerminalIO {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            io_string if io_string.starts_with("$ ls") => Ok(TerminalIO::Ls),
            io_string if io_string.starts_with("$ cd") => {
                let dir_name = io_string.split(" ").last().unwrap();
                Ok(TerminalIO::Cd(dir_name.to_owned()))
            }
            io_string if io_string.as_bytes()[0].is_ascii_digit() => {
                let mut split = io_string.split(" ");
                Ok(TerminalIO::OutputFile {
                    size: split.next().unwrap().parse().unwrap(),
                    name: split.next().unwrap().to_owned(),
                })
            }
            io_string if io_string.as_bytes()[0].is_ascii_alphabetic() => {
                let split = io_string.split(" ");
                Ok(TerminalIO::OutputDir {
                    name: split.last().unwrap().to_owned(),
                })
            }
            _ => Err(()),
        }
    }
}

pub fn parse(input: &str) -> HashMap<String, u32> {
    let io = input
        .lines()
        .map(|l| TerminalIO::from_str(l).unwrap())
        .collect();
    return create_folder_structure(io);
}

pub fn create_folder_structure(terminal_io: Vec<TerminalIO>) -> HashMap<String, u32> {
    let mut current_path = "".to_string();
    let mut folder_sizes: HashMap<String, u32> = HashMap::new();
    for io in terminal_io.iter() {
        match io {
            TerminalIO::Ls => (),
            TerminalIO::Cd(tar) => {
                match tar.as_str() {
                    "/" => current_path = "/".to_string(),
                    ".." => {
                        let split_path: Vec<&str> = current_path.split("/").collect();
                        current_path = split_path[..split_path.len() - 2].join("/") + "/";
                    }
                    _ => current_path += &(tar.to_owned() + "/"),
                };
            }
            TerminalIO::OutputFile { name: _, size } => {
                let mut temp_path = "/".to_string();
                for folder_name in current_path[..current_path.len() - 1].split("/") {
                    temp_path.push_str(folder_name);
                    folder_sizes
                        .entry(temp_path.to_string())
                        .and_modify(|s| *s += size)
                        .or_insert(*size);
                    temp_path += "/";
                }
            }
            TerminalIO::OutputDir { name: _ } => (),
        };
    }
    return folder_sizes;
}
