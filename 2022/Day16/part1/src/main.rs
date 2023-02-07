use std::fs;

fn main() {
    let valves = fs::read_to_string("valve.txt").unwrap();
    let valve_list: Vec<Valve> = valves.split('\n')
        .map(Valve::from)
        .collect();

    println!("{:?}", valve_list);
}

#[derive(Debug)]
struct Valve {
    name: String,
    flow: usize,
    tunnels: Vec<String>,
}

impl From<&str> for Valve {
    fn from(line: &str) -> Self {
        let l: Vec<&str> = line.split(';').collect();
        let n = l[0].split(' ').nth(1).unwrap().to_string();
        let f = l[0].split(' ').last().unwrap().split('=').last().unwrap().parse::<usize>().unwrap();
        let t = l[1].replace(',', "").split(' ')
            .skip_while(|i| !i.contains("valve"))
            .skip(1).map(|t| t.to_string())
            .collect();
        
        Valve {name: n, flow: f, tunnels: t} 
    }
}
