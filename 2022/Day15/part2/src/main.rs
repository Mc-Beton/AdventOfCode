use std::fs;
use std::collections::HashSet;
use std::iter::Iterator;
use part2::Sensor;

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