use std::fs;
use crate::Signal::{Addx, Noop};

fn main() {
    let noops = fs::read_to_string("noop_list.txt").unwrap();
    let sig_list: Vec<Signal> = noops.split('\n')
        .map(Signal::from)
        .collect();

    let signal_list = {
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

    let solution1 = signal_list[19]*20
                    + signal_list[59]*60
                    + signal_list[99]*100
                    + signal_list[139]*140
                    + signal_list[179]*180
                    + signal_list[219]*220;
    
    println!("{}", solution1);
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