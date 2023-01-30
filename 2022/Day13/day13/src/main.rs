use std::fs;
use itertools::Itertools;
use crate::Digit::{Value, List};

fn main() {
    
    let signal = fs::read_to_string("signal.txt").unwrap();
    let mut sig_list = signal.split('\n')
        .filter(|line| !line.trim().is_empty())
        .map(Digit::from)
        .collect_vec();

}

#[derive(Debug, Clone, Eq, PartialEq)]
enum Digit {
    Value(usize),
    List(Vec<Digit>),
}

impl Digit {
    fn to_compare(&self) -> Digit {
        match self {
            Value(v) => Digit::List(vec![Value(*v)]),
            List(v) => Digit::List(v.clone()),
        }
    }
}

impl From<&str> for Digit {
    fn from(line: &str) -> Self {

        let mut buffer = String::new();
        let mut open_vec = Vec::new();

        for c in line.chars() {
            match c {
                num => buffer.push(num),
                '[' => {
                    let new_list = List(vec![]);
                    open_vec.push(new_list);
                },
                ']' => {
                    if !buffer.is_empty() {
                        let num = buffer.parse::<usize>().unwrap();
                        if let Some(List(list)) = open_vec.last_mut() {
                            list.push(Digit::Value(num));
                        } else {
                            unimplemented!()
                        }
                        buffer.clear();
                    }

                    let closed_list = open_vec.pop().unwrap();
                    if let Some(List(list)) = open_vec.last_mut() {
                        list.push(closed_list);
                    } else {
                        return closed_list;
                    }
                },
                ',' => {
                    if !buffer.is_empty() {
                        let num = buffer.parse::<usize>().unwrap();
                        if let Some(List(list)) = open_vec.last_mut() {
                            list.push(Digit::Value(num));
                        } else {
                            unimplemented!()
                        }
                        buffer.clear();
                    }
                },
            }
        };
        unimplemented!();
    }
}