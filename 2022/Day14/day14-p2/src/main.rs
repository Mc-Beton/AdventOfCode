use std::fs;
use itertools::Itertools;
use ingrid::Grid;
use ingrid::{coord, size};
use ingrid::{Coordinate, Size};
use ingrid::GridIterator;

fn main() {
    let rock_build = fs::read_to_string("rocks.txt").unwrap();

    let rock: Vec<Vec<Rocks>> = rock_build.trim().lines()
        .map(|l| l.split("->")
            .map(|r| r.split(",").next_tuple::<(_, _)>().unwrap())
            .map(Rocks::from).collect()).collect();
            
    let x_size: Vec<usize> = rock.clone().into_iter().flatten().map(|rx| rx.x).collect();
    let y_size: Vec<usize> = rock.clone().into_iter().flatten().map(|ry| ry.y).collect();

    let row_size = x_size.iter().max().unwrap();
    let column_size = y_size.iter().max().unwrap();

    let mut rock_map = Grid::with_size(size!(*row_size + 500, *column_size + 3), ".");

    //add the base line
    rock_map.insert_row(*column_size + 2, vec!["#"; *row_size + 500]); 

    rock.iter()
        .for_each(|line| line.iter()
            .tuple_windows::<(_,_)>().for_each(|(a, b)| {
                let x_dif;
                let y_dif;
                if b.x >= a.x { x_dif = b.x - a.x; } 
                else { x_dif = a.x - b.x; };

                if b.y >= a.y { y_dif = b.y - a.y; } 
                else { y_dif = a.y - b.y; };
            
                rock_map[coord!(a.x, a.y)] = "#";

                if x_dif != 0 {
                    (0..x_dif + 1).for_each(|i| {
                        if b.x >= a.x {
                            rock_map[coord!(a.x + i, a.y)] = "#";
                        } else {
                            rock_map[coord!(b.x + i, b.y)] = "#";
                        }     
                    })
                } else if y_dif != 0 {
                    (0..y_dif + 1).for_each(|i| {
                        if b.y >= a.y {
                            rock_map[coord!(a.x, a.y + i)] = "#";
                        } else {
                            rock_map[coord!(b.x, b.y + i)] = "#";
                        }     
                    })
                } else {
                    unreachable!()
                }
        }
    )); 

    //add the source
    rock_map[coord!(500, 0)] = "+";

    let mut sand_pos = (500, 0);
    let mut count = 0;

    loop {
        let down = rock_map.row(sand_pos.1 + 1)[sand_pos.0];
        let down_left = rock_map.row(sand_pos.1 + 1)[sand_pos.0 - 1];
        let down_right = rock_map.row(sand_pos.1 + 1)[sand_pos.0 + 1];

        if down == "o" && down_left == "o" && down_right == "o" && sand_pos.1 == 0 {
            break;

        } else if down == "." || down == " " {
            sand_pos = (sand_pos.0, sand_pos.1 + 1);

        } else if down_left == "." || down_left == " " {
            sand_pos = (sand_pos.0 - 1, sand_pos.1 + 1);
            
        } else if down_right == "." || down_right == " " {
            sand_pos = (sand_pos.0 + 1, sand_pos.1 + 1);
            
        } else {
            rock_map[coord!(sand_pos.0, sand_pos.1)] = "o";
            count += 1;
            sand_pos = (500, 0);
        }
    }

    println!("{}", count);
}

#[derive(Debug, Clone)]
struct Rocks {
    x: usize,
    y: usize,
}

impl From<(&str, &str)> for Rocks {
    fn from(rock: (&str, &str)) -> Self {
        let (r1, r2) = rock;
        Rocks {
            x: r1.trim().parse::<usize>().unwrap(),
            y: r2.trim().parse::<usize>().unwrap(),
        }          
    }
}