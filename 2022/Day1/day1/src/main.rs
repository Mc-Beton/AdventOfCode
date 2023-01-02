use std::fs;

fn main() {
    
    let food = fs::read_to_string("elves.txt").expect("Should read file");
    let mut cal = 0;
    let mut max = 0;
    for line in food.lines() {

        if line.is_empty() {
            if cal > max {
                max = cal;
            }
            cal = 0;
            continue;
        }

        match line.parse::<i64>() {
            Ok(n) => {
                cal += n;
            },
            Err(e) => {
                println!("Error {}", e);
                continue;
            }
        };
    }
    println!("Max: {}", max);
}

