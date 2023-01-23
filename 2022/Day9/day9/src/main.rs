use std::fs;
use itertools::Itertools;
use std::collections::HashSet;
use crate::Directions::{Left, Right, Up, Down};

fn main() {
    let moves = fs::read_to_string("move_list.txt").unwrap();

    let pos_zero = Position {x: 0, y: 0};

    let move_list: Vec<Move> = moves.split('\n')
        .map(|line| line.split_whitespace().next_tuple::<(_, _)>().unwrap())
        .map(Move::from).collect();
}

struct Position {
    x: isize,
    y: isize,
}

struct Move {
    direction: Directions,
    steps: usize,
}

#[derive(Copy, Debug, Clone)]
enum Directions {
    Right,
    Left,
    Up,
    Down,
}

impl Move {
    fn do_move(&self, pos: Position) -> Position {
        match self.direction {
            Right => pos.move_right(self.steps),
            Left => pos.move_left(self.steps),
            Up => pos.move_up(self.steps),
            Down => pos.move_down(self.steps),
            _ => unimplemented!()
        }
    }

    fn move_once(&self) -> Self {
        Move {
            direction: self.direction,
            steps: 1,
        }
    }
}

impl Position {
    fn new_pos(x: isize, y: isize) -> Self {Position {x, y}}

    fn move_right(&self, steps: usize) -> Position {
        Self::new_pos(self.x + steps as isize, self.y)
    }

    fn move_left(&self, steps: usize) -> Position {
        Self::new_pos(self.x - steps as isize, self.y)
    }

    fn move_up(&self, steps: usize) -> Position {
        Self::new_pos(self.x, self.y + steps as isize)
    }

    fn move_down(&self, steps: usize) -> Position {
        Self::new_pos(self.x, self.y - steps as isize)
    }
}

impl From<(&str, &str)> for Move {
    fn from(line: (&str, &str)) -> Self {
        let (dir, steps) = line;
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

