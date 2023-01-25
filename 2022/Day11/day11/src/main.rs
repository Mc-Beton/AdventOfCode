use std::fs;

fn main() {
    let monks = fs::read_to_string("monkey_list.txt").unwrap();
    let monkey_list: Vec<_> = monks.split('\n').collect();

    let mut monkeys: Vec<Monkey> = Vec::new();

    for mon in monkey_list.chunks(7) {
        let m = Monkey {
            items: {
                let l1 = mon[1].replace("Starting items:", "");
                l1.replace(",", "");
                l1.split(' ').map(|n| n.parse::<usize>().unwrap()).collect()   
            },
            operation: {
                let l2: Vec<&str> = mon[2].split(' ').collect();
                let oper = Operation {
                    sub: l2.last().unwrap().parse::<usize>().unwrap(),
                    op: l2.get(l2.len()-1).unwrap().chars().next().expect("this must be a sign"),
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

}

struct Monkey {
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
    fn test_item(&self, item: usize) -> usize {
        item/self.test
    }
}