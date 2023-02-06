use crate::Commands::{ToRoot, GoBack, GoToDir, List, FileIn, DirIn};

#[derive(Debug)]
pub struct File {
    pub size: usize,
    pub name: String,
}

#[derive(Debug)]
pub struct Dir {
    pub name: String,
    pub dirs: Vec<usize>,
    pub files: Vec<usize>,
    
}

pub enum Commands {
    ToRoot,
    GoBack,
    GoToDir(String),
    List,
    FileIn(usize, String),
    DirIn(String),
}

impl From<&str> for Commands {
    fn from(line: &str) -> Self {
        let lines = line.split_whitespace();
        let words = lines.collect::<Vec<&str>>();
        let command = match (words[0], words[1], words.get(2)) {
            ("$", "cd", Some(&"/")) => ToRoot,
            ("$", "cd", Some(&"..")) => GoBack,
            ("$", "cd", Some(name)) => GoToDir(name.to_string()),
            ("$", "ls", _) => List,
            ("dir", name, _) => DirIn(name.to_string()),
            (size, name, _) => FileIn(size.parse::<usize>().unwrap(),
                                        name.to_string()),
        };
        command
    }
}