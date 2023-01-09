use std::fs;

fn hand_shape(p: char) -> i32 {
    match p {
        'X' => 1,
        'Y' => 2,
        'Z' => 3,
        _ => unreachable!(),
    }
}

fn change_hand(o: char, p: char) -> char {
    match p {
        'X' => loose(o),
        'Y' => draw(o),
        'Z' => win(o),
        _ => unreachable!(),
    }
}

fn loose(o: char) -> char {
    match o {
        'A' => 'Z',
        'B' => 'X',
        'C' => 'Y',
        _ => unreachable!(),
    }
}

fn win(o: char) -> char {
    match o {
        'A' => 'Y',
        'B' => 'Z',
        'C' => 'X',
        _ => unreachable!(),
    }
}

fn draw(o: char) -> char {
    match o {
        'A' => 'X',
        'B' => 'Y',
        'C' => 'Z',
        _ => unreachable!(),
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
        let p = change_hand(o, l.last().unwrap());
        //let p = change_hand(o, p);
        sum += hand_shape(p) + round_result(o, p);
    }

    println!("{sum}");

    
}