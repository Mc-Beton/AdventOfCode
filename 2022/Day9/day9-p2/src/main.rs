use std::fs;
use itertools::Itertools;
use std::collections::HashSet;
use day9_p2::{Position, Move};

fn main() {
    let moves = fs::read_to_string("move_list.txt").unwrap();

    let move_list: Vec<Move> = moves.split('\n')
        .map(|line| line.split_whitespace().next_tuple::<(_, _)>().unwrap())
        .map(Move::from).collect();

    let tail_moves = {
        let head = Position::new_pos(0, 0);
        let tail = Position::new_pos(0, 0);
        let mut head_list = Vec::new();
        let mut tail_list = Vec::new();
        head_list.push(head);
        tail_list.push(tail);
        move_list.iter().for_each(|m| {
            for _ in 0..m.steps {
                head_list.push(m.move_once().do_move(*head_list.last().unwrap()));
                if head_list.len() > 10 {
                    tail_list.push(tail_list.last().unwrap()
                        .follow(
                            head_list.last().unwrap(), 
                            head_list.get(head_list.len() - 10).unwrap()))
                } else {
                    tail_list.push(*tail_list.last().unwrap());
                }; 
            };
        });
        tail_list
    };

    let mut solution2 = HashSet::new();
    for i in tail_moves {
        solution2.insert(i);
    }

    println!("{}", solution2.len());
}



