use std::fs;
use std::collections::HashSet;
use std::iter::Iterator;

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

    let mut sig_map = sens_map.clone();

    for i in sensor_list {
        let list = i.get_sensor_area(2000000);
        for j in list {
            sig_map.insert(j);
        }
    }

    let solution1 = get_row_length(sig_map, sens_map, 2000000);
    
    println!("{}", solution1);
}

fn get_row_length(sig: HashSet<(isize, isize)>, set: HashSet<(isize, isize)>, row: isize) -> usize {
    let map = sig.iter()
        .filter(|coor| coor.1 == row)
        .count();

    let beacons = set.iter()
        .filter(|coor| coor.1 == row)
        .count();

    map - beacons
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

impl Sensor {

    fn get_distance_to_beacon(&self) -> isize {
        (self.sens.0 - self.beac.0).abs() + (self.sens.1 - self.beac.1).abs()
    }

    fn get_sensor_area(&self, row: isize) -> Vec<(isize, isize)> {
        let dist = self.get_distance_to_beacon();
        let mut area = Vec::new();

        if ((self.sens.1 - dist)..self.sens.1).contains(&row) || ((self.sens.1)..(self.sens.1 + dist)).contains(&row) {
            let dif = (dist - (row - self.sens.1).abs()).abs();
            for i in (-dif)..(dif+1) {
                area.push((self.sens.0 + i, row));
            }
        }
        area
    }
}
