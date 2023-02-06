use std::fs;
use std::collections::HashSet;

fn main() {
    let packet_list = fs::read_to_string("packet_list.txt").unwrap();

    fn find_first_marker(input: &str, size: usize) -> usize {
        input.chars().collect::<Vec<char>>()
            .windows(size)
            .map(|chars| chars.iter().collect::<HashSet<&char>>())
            .enumerate()
            .find(|(_, set)| set.len() == size)
            .map(|(index, _)| index + size)
            .unwrap()
    }

    let answer1 = find_first_marker(&packet_list, 4);
    let answer2 = find_first_marker(&packet_list, 14);
    println!("{}", answer1);
    println!("{}", answer2);
}
