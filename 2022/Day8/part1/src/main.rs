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

    let solution1 = grid.iterator().enumerate_coordinate().map(|(c, tree)|{
        let we = grid.row(c.y).iterator().map(|num| num).collect_vec();
        let (w, e) = we.split_at(c.x);
        let ns = grid.column(c.x).iterator().map(|num| num).collect_vec();
        let (n, s) = ns.split_at(c.y);

        let from_west = w.iter().max().map(|t|t < &tree).unwrap_or(true);
        let from_east = e.iter().skip(1).max().map(|t|t < &tree).unwrap_or(true);
        let from_north = n.iter().max().map(|t|t < &tree).unwrap_or(true);
        let from_south = s.iter().skip(1).max().map(|t|t < &tree).unwrap_or(true);

        usize::from(from_west || from_east || from_north || from_south)
    }).sum::<usize>();

    println!("{}", solution1);
}