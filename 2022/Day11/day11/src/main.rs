use std::fs;

fn main() {
    let monkeys = fs::read_to_string("mokey_list.txt").unwrap();
}

struct Monkey {
    num: usize,
    items: Vec<usize>,
    operation: Operation,
    test: usize,
    pass: usize,
    fail: usize,
}

struct Operation {
    sub: usize,
    op: char
}

impl Operation {
    fn do_op(&self, old: usize) -> usize {
        match self.op {
            '+' => old + self.sub,
            '-' => old - self.sub,
            '*' => old * self.sub,
            '/' => old / self.sub,
            _ => 0,
        }
    }
}

impl Monkey {
    fn test_item(&self, it: usize) -> usize {
        it/self.test
    }
}


