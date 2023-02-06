#[derive(Debug)]
pub struct Sensor {
    pub sens: (isize, isize),
    pub beac: (isize, isize),
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

    pub fn get_distance_to_beacon(&self) -> isize {
        (self.sens.0 - self.beac.0).abs() + (self.sens.1 - self.beac.1).abs()
    }

    pub fn get_sensor_area(&self, row: isize) -> Vec<(isize, isize)> {
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