use std::fs;

fn main() {

    let movement_list = fs::read_to_string("movement_list.txt").unwrap();
    let moves: Vec<Move> = movement_list.split("\n\n").nth(1).unwrap().split('\n')
        .map(|line|{
            let lines = line.split_whitespace();
            let words = lines.collect::<Vec<&str>>();
            Move {
                from: words[3].parse().unwrap(),
                to: words[5].parse().unwrap(),
                amount: words[1].parse().unwrap()
            }
        }).collect();

    let start_list = vec![
        vec!['Q','W','P','S','Z','R','H','D'],
        vec!['V','B','R','W','Q','H','F'],
        vec!['C','V','S','H'],
        vec!['H','F','G'],
        vec!['P','G','J','B','Z'],
        vec!['Q','T','J','H','W','F','L'],
        vec!['Z','T','W','D','L','V','J','N'],
        vec!['D','T','Z','C','J','G','H','F'],
        vec!['W','P','V','M','B','H'],
        ];

    let mut answer = start_list.clone();
    moves.iter().for_each(|m| {
        let sec_len = answer[m.from-1].len();
        let tail = answer[m.from-1].split_off(sec_len-m.amount); 
        answer[m.to-1].extend(tail);
    });

    let mut end_list: Vec<char> = vec![];
    end_list
        .extend(answer
            .iter()
            .map(|crates| crates.last().unwrap()));
                
    println!("{:?}", end_list);
}

struct Move {
    amount: usize,
    from: usize,
    to: usize
}