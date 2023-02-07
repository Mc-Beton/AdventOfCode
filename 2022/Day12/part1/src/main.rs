use std::fs;
use crate::Signature::{Start, End, Basic};
use ndarray::{Array2, Ix};

fn main() {
    let input = fs::read_to_string("map.txt").unwrap();

    let rows: Vec<&str> = input.split('\n').collect();
    let width = rows.first().unwrap().len();
    let height = rows.len();

    let chars: Vec<Location> = input.replace('\n', "").chars().map(Location::from).collect();

    let map = Array2::from_shape_vec((height, width), chars).unwrap();

    let start = map.indexed_iter()
        .find(|(_, loc)| loc.find_start())
        .map(|(pos, loc)| Node {pos, loc: loc.clone()})
        .unwrap();

    let end = map.indexed_iter()
        .find(|(_, loc)| loc.find_end())
        .map(|(pos, loc)| Node {pos, loc: loc.clone()})
        .unwrap();
}

#[derive(Debug, Clone)]
struct Node {
    pos: (Ix, Ix),
    loc: Location, 
}

#[derive(Debug, Clone)]
enum Signature {
    Start,
    End,
    Basic,
}

#[derive(Debug, Clone)]
struct Location {
    signature: Signature,
    value: char,
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
    fn get_value_as_int(&self) -> usize {
        match self.value {
            'a'..='z' => self.value as usize - 'a' as usize + 1,
            _ => 0,
        }
    }

    fn find_start(&self) -> bool {
        matches!(self.signature, Start)
    }

    fn find_end(&self) -> bool {
        matches!(self.signature, End)
    }
}