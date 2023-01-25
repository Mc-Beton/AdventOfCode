use std::fs;

fn main() {
    let monks = fs::read_to_string("monkey_list.txt").unwrap();
    let monkey_list: Vec<_> = monks.split('\n').collect();

    let mut monkeys: Vec<Monkey> = Vec::new();

    for mon in monkey_list.chunks(7) {
        let m = Monkey {
            items: {
                let l1 = mon[1].replace("Starting items:", "");
                let l = l1.replace(",", "");
                l.split_whitespace().map(|n| n.parse::<usize>().unwrap()).collect()  
            },
            operation: {
                let l2: Vec<&str> = mon[2].split(' ').collect();
                let oper = Operation {
                    sub: l2.last().unwrap(),
                    op: l2.get(l2.len()-2).unwrap().chars().next().expect("this must be a sign"),
                };
                oper
            },
            test: {
                let l3: Vec<&str> = mon[3].split(' ').collect();
                l3.last().unwrap().parse::<usize>().unwrap()
            },
            pass: {
                let l4: Vec<&str> = mon[4].split(' ').collect();
                l4.last().unwrap().parse::<usize>().unwrap()
            },
            fail: {
                let  l5: Vec<&str> = mon[5].split(' ').collect();
                l5.last().unwrap().parse::<usize>().unwrap()
            },
        };
        monkeys.push(m);
    }
    let mut after_20_rounds: Vec<Monkey> = {
        for _ in 0..19 {
            monkeys.iter().map(|m| m)
        }   
    }
}

#[derive(Debug)]
struct Monkey<'a> {
    items: Vec<usize>,
    operation: Operation<'a>,
    test: usize,
    pass: usize,
    fail: usize,
}

#[derive(Debug)]
struct Operation<'a> {
    sub: &'a str,
    op: char
}

impl Operation<'_> {
    fn do_op(&self, old: usize) -> usize {
        let mut sub = 0;
        if self.sub.to_string() == "old" { sub = old; } 
        else { sub = self.sub.parse::<usize>().unwrap(); }

        match self.op {
            '+' => old + sub,
            '-' => old - sub,
            '*' => old * sub,
            '/' => old / sub,
            _ => 0,
        }
    }
}

impl Monkey<'_> {
    fn test_item(&self, item: usize) -> usize {
        item/self.test
    }
}