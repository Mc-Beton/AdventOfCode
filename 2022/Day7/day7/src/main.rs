use std::fs;
use crate::Commands::{ToRoot, GoBack, GoToDir, List, FileIn, DirIn};

fn main() {
    
    let command_list = fs::read_to_string("command_list.txt").unwrap();
    let commands: Vec<Commands> = command_list.split('\n')
        .map(Commands::from)
        .collect();

    let mut dirs = vec![Dir {
        name: String::from("/"),
        dirs: Vec::new(),
        files: Vec::new(),
    }];

    let mut files: Vec<File> = Vec::new();

    let mut current_dir_idx: usize = 0;

    commands.iter().for_each(|com| {
        match com {
            ToRoot => {},

            DirIn (name) => {   let new_dir = Dir {
                                    name: name.to_string(),
                                    dirs: Vec::new(),
                                    files: Vec::new(),
                                };
                                dirs.push(new_dir);
                                let new_dir_idx = dirs.len() - 1;
                                dirs[current_dir_idx]
                                    .dirs
                                    .push(new_dir_idx); },

            FileIn (size, name) => {let new_file = File {
                                        size: *size,
                                        name: name.to_string(),
                                        };
                                    files.push(new_file); 
                                    dirs[current_dir_idx]
                                        .files
                                        .push(files.len() - 1);},

            GoBack => {
                        for (i, dir) in dirs.iter().enumerate() {
                            if dir.dirs.contains(&current_dir_idx) {
                                current_dir_idx = i;
                                break;
                            }
                        }
                    },
            List => {},

            GoToDir (path_name) => {
                                    for dir_idx in &dirs[current_dir_idx].dirs {
                                        if dirs[*dir_idx].name == path_name.to_string() {
                                            current_dir_idx = *dir_idx;
                                        }
                                    }
                    }
        }
    });

    let mut dir_size = vec![0; dirs.len()];

    for (i, dir) in dirs.iter().enumerate().rev() {
        for file_id in &dir.files {
            dir_size[i] += files[*file_id].size;

        }

        for dir_id in &dir.dirs {
            dir_size[i] += dir_size[*dir_id];
        }
    }

    let mut solution1 = 0;

    for size in dir_size.iter() {
        if *size < 100000 {
            solution1 += size;
        }
    }

    println!("{}", solution1);
}

#[derive(Debug)]
struct File {
    size: usize,
    name: String,
}

#[derive(Debug)]
struct Dir {
    name: String,
    dirs: Vec<usize>,
    files: Vec<usize>,
    
}

enum Commands {
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
