use std::fs;
use std::collections::HashSet;

fn main() {
    let packet_list = fs::read_to_string("packet_list.txt").unwrap();

    fn find_first_marker(input: String) -> usize {
        input.chars().collect::<Vec<char>>()
            .windows(4)
            .map(|chars| chars.iter().collect::<HashSet<&char>>())
            .enumerate()
            .find(|(_, set)| set.len() == 4)
            .map(|(index, _)| index + 4)
            .unwrap()
    }

    let answer = find_first_marker(packet_list);
    println!("{}", answer);
}
