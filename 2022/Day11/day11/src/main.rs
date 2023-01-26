use std::fs;
use itertools::Itertools;
use day11::{Operation, Monkey};

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
            inspections: 0,
        };
        monkeys.push(m);
    }

    let after_20_rounds: Vec<Monkey> = {
        for _ in 0..20 {
            for i in 0..monkeys.len() {
                let monkey = &mut monkeys[i];
                let mut items_to_send = Vec::new();
                while monkey.items.len() > 0 {
                    let item = monkey.items.remove(0);
                    monkey.inspections += 1;

                    let worry_lvl = monkey.operation.get_worry_level(item);

                    let m_id = match monkey.test_item(worry_lvl) {
                        0 => monkey.pass,
                        _ => monkey.fail,
                    };
                    items_to_send.push((m_id, worry_lvl));
                };
                for (m_id, item) in items_to_send {
                    monkeys[m_id].items.push(item);
                }
            };
        };
        monkeys   
    };

    let mut monkeys_sorted = after_20_rounds.iter().map(|m| m.inspections).collect_vec();
    monkeys_sorted.sort();
    let solution1 = monkeys_sorted.last().unwrap() * monkeys_sorted.get(monkeys_sorted.len() - 2).unwrap();

    println!("{}", solution1);
}