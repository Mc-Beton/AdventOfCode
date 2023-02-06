use std::fs;

fn main() {
    let task: Vec<Vec<Vec<usize>>> = fs::read_to_string("task_sheet.txt").unwrap()
    .split('\n')
    .map(|line| {
        line.split(',')
            .map(|pair| {
                pair.split('-')
                    .map(|sec| sec.parse().unwrap()).collect()
            }).collect()
    }).collect();

    println!("{}", get_both_elves_sec(task));
}

fn get_both_elves_sec(list: Vec<Vec<Vec<usize>>>) -> usize {
    list
        .iter()
        .filter(|&pairs| match (pairs.get(0), pairs.get(1)) {
            (Some(p1), Some(p2)) => check_sec_time(p1, p2) || check_sec_time(p2, p1),
            _ => false,
        })
        .count()
}

fn check_sec_time(pair1: &Vec<usize>, pair2: &Vec<usize>) -> bool {
    if let (Some(&s1), Some(&e1), Some(&s2), Some(&e2)) = 
        (pair1.get(0), pair1.get(1), pair2.get(0), pair2.get(1)) {
            return s1 <= e2 && s2 <= e1
        }
        false
}