use std::fs;
use crate::Directions::{Left, Right, Up, Down};

fn main() {
    let rope_move = fs::read_to_string("move_list.txt").unwrap();
    
}

struct Move {
    direction: Directions,
    steps: usize,
}

enum Directions {
    Right,
    Left,
    Up,
    Down,
}

impl From<(&str, &str)> for Move {
    fn from(lines: (&str, &str)) -> Self {
        let (dir, steps) = lines;
        Move {
            direction: match dir {
                "R" => Right,
                "L" => Left,
                "U" => Up,
                "D" => Down,
                _ => unimplemented!()
            },
            steps: steps.parse::<usize>().unwrap(),
        }
    }
}

