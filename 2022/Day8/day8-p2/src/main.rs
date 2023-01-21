use std::fs;
use itertools::Itertools;
use ingrid::{Grid, GridIterator};

fn main() {
    let tree_map = fs::read_to_string("tree_map.txt").unwrap();

    let grid = Grid::from_rows( {
        tree_map.lines()
            .map(|l| l.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect_vec())
            .collect_vec()
    });

    let solution2 = grid.iterator().enumerate_coordinate().map(|(c, tree)|{
        let we = grid.row(c.y).iterator().map(|num| num).collect_vec();
        let (w, e) = we.split_at(c.x);
        let ns = grid.column(c.x).iterator().map(|num| num).collect_vec();
        let (n, s) = ns.split_at(c.y);

        let from_west = w.iter().rev().enumerate().find(|&(_, t)| t >= &tree ).map(|(i, _)| i+1).unwrap_or(w.len());
        let from_east = e.iter().skip(1).enumerate().find(|&(_, t)| t >= &tree ).map(|(i, _)| i+1).unwrap_or(e.len()-1);
        let from_north = n.iter().rev().enumerate().find(|&(_, t)| t >= &tree ).map(|(i, _)| i+1).unwrap_or(n.len());
        let from_south = s.iter().skip(1).enumerate().find(|&(_, t)| t >= &tree ).map(|(i, _)| i+1).unwrap_or(s.len()-1);

        from_east * from_north * from_south * from_west   
        
    }).max().unwrap();

    println!("{}", solution2);
}