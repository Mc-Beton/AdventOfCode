use std::fs;
use itertools::Itertools;

fn main() {
    let rock_build = fs::read_to_string("rocks.txt").unwrap();

    let rock: Vec<Vec<Rocks>> = rock_build.trim().lines()
        .map(|l| l.split("->").
            map(|r| r.split(",").next_tuple::<(_, _)>().unwrap())
            .map(Rocks::from).collect()).collect();    
            
    println!("{:#?}", rock);
}

#[derive(Debug)]
struct Rocks {
    x: usize,
    y: usize,
}

impl From<(&str, &str)> for Rocks {
    fn from(rock: (&str, &str)) -> Self {
        let (r1, r2) = rock;
        Rocks {
            x: r1.trim().parse::<usize>().unwrap(),
            y: r2.trim().parse::<usize>().unwrap(),
        }          
    }
}