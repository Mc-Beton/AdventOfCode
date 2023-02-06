use std::fs;
use crate::Signal::{Addx, Noop};

fn main() {
    let noops = fs::read_to_string("noop_list.txt").unwrap();
    let sig_list: Vec<Signal> = noops.split('\n')
        .map(Signal::from)
        .collect();

    let sprite_pos = {
        let mut list: Vec<isize> = Vec::new();
        list.push(1);
        sig_list.iter().for_each(|s| {
            match s {
                Addx (num) => {
                    list.push(*list.last().unwrap());
                    list.push(*list.last().unwrap() + num);
                },
                Noop => list.push(*list.last().unwrap()),
            };
        });
        list
    };

    let solution2 = { 
        let mut mes = String::new();
        let iter = sprite_pos.chunks(40);
        for l in iter {
            l.iter().enumerate().for_each(|(i, num)| {
                mes.push(write_sign(i.try_into().unwrap(),*num))
            });
            mes.push('\n');
        };
        mes
    };

    println!("{}", solution2);  
}

fn write_sign(i: isize, num: isize) -> char {
    if i >= num - 1 && i <= num + 1 {
        '█'
    } else {
        ' '
    }
}

enum Signal {
    Noop,
    Addx(isize),
}

impl From<&str> for Signal {
    fn from(line: &str) -> Self {
        let lines = line.split_whitespace();
        let words = lines.collect::<Vec<&str>>();
        let sig = match (words[0], words.get(1)) {
            ("noop", _) => Noop,
            ("addx", Some(num)) => Addx(num.parse::<isize>().unwrap()),
            (_, _) => unimplemented!(),
        };
        sig
    }
}