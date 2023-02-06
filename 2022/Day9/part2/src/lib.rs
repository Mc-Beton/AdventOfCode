use crate::Directions::{Left, Right, Up, Down};

#[derive(Copy, Debug, Clone)]
pub enum Directions {
    Right,
    Left,
    Up,
    Down,
}

#[derive(Eq, Hash, PartialEq, Copy, Clone, Debug)]
pub struct Position {
    pub x: isize,
    pub y: isize,
}

impl Position {
    pub fn new_pos(x: isize, y: isize) -> Self {Position {x, y}}

    pub fn move_right(&self, steps: usize) -> Position {
        Self::new_pos(self.x + steps as isize, self.y)
    }

    pub fn move_left(&self, steps: usize) -> Position {
        Self::new_pos(self.x - steps as isize, self.y)
    }

    pub fn move_up(&self, steps: usize) -> Position {
        Self::new_pos(self.x, self.y + steps as isize)
    }

    pub fn move_down(&self, steps: usize) -> Position {
        Self::new_pos(self.x, self.y - steps as isize)
    }

    pub fn calculate_distance(&self, head: &Position) -> Position {
        Self::new_pos(head.x - self.x, head.y - self.y)
    }

    pub fn follow(&self, head: &Position, tail: &Position) -> Position {
        let distance = self.calculate_distance(head);
        let tail_dis = self.calculate_distance(tail);
        if distance.x.abs() >= 9 || distance.y.abs() >= 9 {
            Position::new_pos(self.x + tail_dis.x.signum(), self.y + tail_dis.y.signum())
        } else {
            *self
        }
    }
}

pub struct Move {
    pub direction: Directions,
    pub steps: usize,
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

impl Move {
    pub fn do_move(&self, pos: Position) -> Position {
        match self.direction {
            Right => pos.move_right(self.steps),
            Left => pos.move_left(self.steps),
            Up => pos.move_up(self.steps),
            Down => pos.move_down(self.steps),
        }
    }

    pub fn move_once(&self) -> Self {
        Move {
            direction: self.direction,
            steps: 1,
        }
    }
}