use std::fs;
use part2::{Location, Node};
use ndarray::{Array2};
use pathfinding::prelude::astar;
use part2::{find_neighbours, heuristic};

fn main() {
    let input = fs::read_to_string("map.txt").unwrap();

    let rows: Vec<&str> = input.split('\n').collect();
    let width = rows.first().unwrap().len();
    let height = rows.len();

    let chars: Vec<Location> = input.replace('\n', "").chars().map(Location::from).collect();

    let map = Array2::from_shape_vec((height, width), chars).unwrap();

    let start: Vec<Node> = map.indexed_iter()
        .filter(|(_, loc)| loc.get_value_as_int() == 1)
        .map(|(pos, loc)| Node {pos, loc: loc.clone()})
        .collect();

    let end = map.indexed_iter()
        .find(|(_, loc)| loc.find_end())
        .map(|(pos, loc)| Node {pos, loc: loc.clone()})
        .unwrap();

    let solution2 = start.iter().filter_map(|c| {
        astar(c, |n| find_neighbours(n, &map), |n| heuristic(n, &end), |n| n.loc.find_end())
            .map(|(_, cost)| cost)
        }).min().unwrap();

    println!("{}", solution2);
}