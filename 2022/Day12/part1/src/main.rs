use std::fs;
use crate::Signature::{Start, End, Basic};

fn main() {
    let input = fs::read_to_string("map.txt").unwrap();

    let rows: Vec<&str> = input.split('\n').collect();
    let width = rows.first().unwrap().len();
    let height = rows.len();

    let chars: Vec<Location> = input.replace('\n', "").chars().map(Location::from).collect();
}

enum Signature {
    Start,
    End,
    Basic,
}

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
