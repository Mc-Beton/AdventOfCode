use std::fs;
use std::collections::HashSet;

fn main() {
    let sensors = fs::read_to_string("sensors.txt").unwrap();
    let sensor_list: Vec<Sensor> = sensors.split('\n')
        .map(Sensor::from)
        .collect();

    let mut sens_map = HashSet::new();
    for s in &sensor_list {
        sens_map.insert(s.sens);
        sens_map.insert(s.beac);
    }

    println!("{:?}", sens_map);
}

#[derive(Debug)]
struct Sensor {
    sens: (isize, isize),
    beac: (isize, isize),
}

impl From<&str> for Sensor {
    fn from(line: &str) -> Self {
        let coors: Vec<&str> = line.split(':').collect();
        let sx = coors[0].split(' ').nth(2).unwrap().split('=').nth(1).unwrap().replace(",", "").parse::<isize>().unwrap();
        let sy = coors[0].split(' ').last().unwrap().split('=').nth(1).unwrap().parse::<isize>().unwrap();
        let bx = coors[1].trim().split(' ').nth(4).unwrap().split('=').nth(1).unwrap().replace(",", "").parse::<isize>().unwrap();
        let by = coors[1].trim().split(' ').last().unwrap().split('=').nth(1).unwrap().parse::<isize>().unwrap();
        Sensor {sens: (sx, sy), beac: (bx, by)}
    }
}
