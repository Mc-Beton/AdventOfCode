use std::fs;
use part1::{Location, Node};
use ndarray::{Array2};
use pathfinding::prelude::astar;
use part1::{find_neighbours, heuristic};

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

    let solution1 = astar(&start, |n| find_neighbours(n, &map), |n| heuristic(n, &end), |n| n.loc.find_end())
        .map(|(_, cost)| cost)
        .unwrap();

    println!("{}", solution1);
}