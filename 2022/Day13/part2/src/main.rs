use std::fs;
use itertools::Itertools;
use day13_p2::Digit;
use day13_p2::Digit::{List, Value};
use day13_p2::Comp;
use std::cmp::Ordering;

fn main() {
    
    let signal = fs::read_to_string("signal.txt").unwrap();
    let mut sig_list = signal.split('\n')
        .filter(|line| !line.trim().is_empty())
        .map(Digit::from)
        .collect_vec();

    let div_packet1 = List(vec![List(vec![Value(2)])]);
    let div_packet2 = List(vec![List(vec![Value(6)])]);
    sig_list.push(div_packet1.clone());
    sig_list.push(div_packet2.clone());

    sig_list.sort_by(|a, b| {
        match a.compare(&b) {
            Comp::Equal => Ordering::Equal,
            Comp::Valid => Ordering::Less,
            Comp::Wrong => Ordering::Equal,
        }
    });

    let solution2 = sig_list.iter().enumerate()
        .filter(|(_, d)| {
            d == &&div_packet1 || d == &&div_packet2
        }).map(|(i, _)| {
            i + 1
        }).reduce(|a, b| a * b).unwrap();
    
    println!("{}", solution2);

}