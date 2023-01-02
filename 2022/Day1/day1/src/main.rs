use std::fs;

fn main() {
    
    let food = fs::read_to_string("elves.txt").expect("Should read file");
    for line in food.lines() {
        println!("{}", line);
    }
}
