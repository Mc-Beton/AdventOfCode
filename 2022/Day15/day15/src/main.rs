use std::fs;
use regex::Regex;

fn main() {
    let sensors = fs::read_to_string("sensors.txt").unwrap();
    let re = Regex::new(r"Sensor at x=(\d+), y=(\d+): closest beacon is at x=(\d+), y=(\d+)").unwrap();
    let mut sensor_list = Vec::new();
    for c in re.captures_iter(&sensors) {
        let sensor = Sensor {
            x: c[1].parse().unwrap(),
            y: c[2].parse().unwrap(),
            b_x: c[3].parse().unwrap(),
            b_y: c[4].parse().unwrap(),
        };
        sensor_list.push(sensor); 
    };
}

struct Sensor {
    x: isize,
    y: isize,
    b_x: isize,
    b_y: isize,
}
