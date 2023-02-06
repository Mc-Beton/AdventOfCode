use std::fs;

fn main() {

    let compartment = fs::read_to_string("compartment_sheet.txt").expect("Should read file");

    let mut sum = 0;

    let data: Vec<_> = compartment.lines().collect();

    for line in data.chunks(3) {

        let a: String = line[0].to_string();
        let b: String = line[1].to_string();
        let c: String = line[2].to_string();
        
        let duplicate = compare_comp(&a, &b, &c).unwrap();
        let priority = match duplicate {
            'a'..='z' => duplicate as u8 - 'a' as u8 + 1,
            'A'..='Z' => duplicate as u8 - 'A' as u8 + 27,
            _ => 0,
        } as i32;
        sum += priority;
    }
    println!("{sum}");
}

fn compare_comp(comp1: &str, comp2: &str, comp3: &str) -> Option<char> {
    for c1 in comp1.chars() {
        for c2 in comp2.chars() {
            for c3 in comp3.chars() {
                if c1==c2 && c1==c3 {
                    return Some(c1);
                }
            }
        }
    }
    None
}
