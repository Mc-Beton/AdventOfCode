use crate::Digit::{Value, List};
use itertools::{Itertools, EitherOrBoth};

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Digit {
    Value(usize),
    List(Vec<Digit>),
}

impl Digit {
    pub fn to_compare(&self) -> Digit {
        match self {
            Value(v) => Digit::List(vec![Value(*v)]),
            List(v) => Digit::List(v.clone()),
        }
    }

    pub fn compare(&self, r_side: &Digit) -> Comp {

        let c = match (self, r_side) {
            (Value(l), Value(r)) => match (l,r) {
                (l,r) if l < r => Comp::Valid,
                (l,r) if l > r => Comp::Wrong,
                (l,r) if l == r => Comp::Equal,
                _ => unimplemented!(),
            },
            (Value(_), List(_)) => self.to_compare().compare(r_side),
            (List(_), Value(_)) => self.compare(&r_side.to_compare()),
            (List(l), List(r)) => {
                let double_iter = l.iter().zip_longest(r.iter());

                let mut result = false;
                for pair in double_iter {
                    match pair {
                        EitherOrBoth::Both(l,r) => {
                            let c = l.compare(r);
                            match c {
                                Comp::Equal => {},
                                Comp::Wrong => return c,
                                Comp::Valid => return c,
                            }
                        }
                        EitherOrBoth::Right(_) => {
                            result = true;
                        },
                        EitherOrBoth::Left(_) => {
                            if !result {
                                return Comp::Wrong
                            }
                        }
                    }
                }
                if result {
                    Comp::Valid
                } else {
                    Comp::Equal
                }
            }, 
        };
        c
    }
}


impl From<&str> for Digit {
    fn from(line: &str) -> Self {

        let mut buffer = String::new();
        let mut open_vec = Vec::new();

        for c in line.chars() {
            match c {
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
                num => buffer.push(num),
            }
        };
        unimplemented!();
    }
}

pub enum Comp {
    Valid,
    Wrong,
    Equal,
}