use std::fs;

fn hand_shape(h: char) -> i32 {
    match h {
        'X' => 1,
        'Y' => 2,
        'Z' => 3,
        _ => 0,
    }
}

fn round_result(o: char, p: char) -> i32 {
    match (o,p) {
        ('A', 'Y') | ('B', 'Z') | ('C', 'X') => 6,
        ('A', 'X') | ('B', 'Y') | ('C', 'Z') => 3,
        ('A', 'Z') | ('B', 'X') | ('C', 'Y') => 0,
        _ => 0,
    }
}

fn main() {
    
    let cheet = fs::read_to_string("cheet_sheet.txt").expect("Should read file");
    let mut sum = 0;

    for l in cheet.lines() {
        let mut l = l.chars();
        let o = l.next().unwrap();
        let p = l.last().unwrap();
        sum = sum + hand_shape(p) + round_result(o, p);
    }

    println!("{sum}");

    
}