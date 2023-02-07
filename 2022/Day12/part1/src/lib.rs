use crate::Signature::{Start, End, Basic};
use ndarray::{Array2, Ix};

pub fn find_neighbours(n: &Node, map: &Array2<Location>) -> Vec<(Node, usize)> {
    let up = (n.pos.0 as isize + 1, n.pos.1 as isize);
    let down = (n.pos.0 as isize - 1, n.pos.1 as isize);
    let left = (n.pos.0 as isize, n.pos.1 as isize + 1);
    let right = (n.pos.0 as isize, n.pos.1 as isize - 1);

    let nodes: Vec<(Node, usize)> = vec![up, down, left, right].iter()
        .map(|(x, y)| (*x as Ix, *y as Ix))
        .filter_map(|c| map.get(c).map(|loc| {
            Node {pos: c, loc: loc.clone() }
        }))
        .filter(|s| (s.loc.get_value_as_int() as isize - n.loc.get_value_as_int() as isize) <= 1)
        .map(|n| (n, 1))
        .collect();

    nodes
}

pub fn heuristic(n1: &Node, n2: &Node) -> usize {
    (((n2.pos.0 as isize - n1.pos.0 as isize).pow(2) + (n2.pos.1 as isize - n1.pos.1 as isize).pow(2)) as f64).sqrt() as usize
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct Node {
    pub pos: (Ix, Ix),
    pub loc: Location, 
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub enum Signature {
    Start,
    End,
    Basic,
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct Location {
    pub signature: Signature,
    pub value: char,
}

impl From<char> for Location {
    fn from(c: char) -> Self {
        match c {
            'S' => Location{signature: Start, value: 'a'},
            'E' => Location{signature: End, value: 'z'},
            b => Location{signature: Basic, value: b},
        }
    }
}

impl Location {
    pub fn get_value_as_int(&self) -> usize {
        match self.value {
            'a'..='z' => self.value as usize - 'a' as usize + 1,
            _ => 0,
        }
    }

    pub fn find_start(&self) -> bool {
        matches!(self.signature, Start)
    }

    pub fn find_end(&self) -> bool {
        matches!(self.signature, End)
    }
}