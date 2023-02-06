use std::fs;

fn main() {

    let compartment = fs::read_to_string("compartment_sheet.txt").expect("Should read file");

    let mut sum = 0;

    for l in compartment.lines()  {
        let (comp1, comp2) = l.split_at(l.len()/2);
        let duplicate = compare_comp(comp1, comp2).unwrap();
        let priority = match duplicate {
            'a'..='z' => duplicate as u8 - 'a' as u8 + 1,
            'A'..='Z' => duplicate as u8 - 'A' as u8 + 27,
            _ => 0,
        } as i32;
        sum += priority;
    }
    println!("{sum}");
}

fn compare_comp(comp1: &str, comp2: &str) -> Option<char> {
    for c1 in comp1.chars() {
        for c2 in comp2.chars() {
            if c1==c2 {
                return Some(c1);
            }
        }
    }
    None
}
