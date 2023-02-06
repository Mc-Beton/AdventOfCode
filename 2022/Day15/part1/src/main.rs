use std::fs;
use std::collections::HashSet;

fn main() {
    let sensors = fs::read_to_string("sensors.txt").unwrap();

    let re = Regex::new(r"Sensor at x=(\d+), y=(\d+): closest beacon is at x=(\d+), y=(\d+)").unwrap();

    let mut sensor_list = Vec::new();

    for c in re.captures_iter(&sensors) {

        let sensor = Sensor {
            sens: (c[1].parse().unwrap(), c[2].parse().unwrap()),
            beac: (c[3].parse().unwrap(), c[4].parse().unwrap()),
        };

        println!("{:?}", sensor);
        sensor_list.push(sensor); 
    };

    let mut sens_map = HashSet::new();
    for s in &sensor_list {
        sens_map.insert(s.sens);
        sens_map.insert(s.beac);
    }

    //println!("{:?}", sens_map);
}

#[derive(Debug)]
struct Sensor {
    sens: (isize, isize),
    beac: (isize, isize),
}
