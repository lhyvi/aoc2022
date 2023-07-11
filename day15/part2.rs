use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::ops::Add;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let map = Map::parse_input(input);
    map.get_tuning_frequency(4000000);
}

#[derive(Debug, Hash, Copy, Clone, Eq, PartialEq)]
struct Point {
    x: isize,
    y: isize,
}
impl Point {
    fn new(num1: isize, num2: isize) -> Self {
        Point { x: num1, y: num2 }
    }
    fn from_tuple(numbers: (isize, isize)) -> Self {
        Point {
            x: numbers.0,
            y: numbers.1,
        }
    }
    fn get_distance(&self, other: &Self) -> isize {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
#[derive(Debug)]
struct Map {
    map: Vec<(Point, Point)>,
}

impl Map {
    fn parse_input(input: String) -> Self {
        let mut new_map = vec![];
        for line in input.lines() {
            let values = line.split(' ').collect::<Vec<_>>();
            let sensor_x = parse_to_value(&values[2]);
            let sensor_y = parse_to_value(&values[3]);
            let beacon_x = parse_to_value(&values[8]);
            let beacon_y = parse_to_value(&values[9]);
            let sensor = Point::new(sensor_x, sensor_y);
            let beacon = Point::new(beacon_x, beacon_y);
            new_map.push((sensor, beacon));
        }
        Map { map: new_map }
    }
    fn get_impossible_beacons(&self, row: isize) -> isize {
        let mut count = 0;
        let mut locations = HashSet::new();
        for (_, beacon) in &self.map {
            if beacon.y == row {
                locations.insert(beacon.x);
            }
        }
        for (sensor, beacon) in &self.map {
            let sensor_range = sensor.get_distance(&beacon);
            let sensor_distance = (sensor.y - row).abs();
            if sensor_range >= sensor_distance {
                let coverage = sensor.x - sensor_range + sensor_distance
                    ..=sensor.x + sensor_range - sensor_distance;
                for x in coverage {
                    if locations.insert(x) {
                        count += 1;
                    }
                }
            }
        }

        count
    }
    fn get_tuning_frequency(&self, max: isize) {
        let mut possible_locations = HashSet::new();
        for (sensor, beacon) in &self.map {
            let range = sensor.get_distance(&beacon) + 1;
            for i in 0..=range {
                possible_locations.insert(Point::new(sensor.x + range - i, sensor.y + i));
                possible_locations.insert(Point::new(sensor.x - range + i, sensor.y - i));
            }
        }
        'outer: for location in possible_locations {
            if location.x > max || location.x < 0 || location.y > max || location.y < 0 {
                continue 'outer;
            }
            for (sensor, beacon) in &self.map {
                let range = sensor.get_distance(&beacon);
                let distance = sensor.get_distance(&location);
                if distance <= range {
                    continue 'outer;
                }
            }
            let result = (location.x as isize * 4000000) + location.y as isize;
            println!("{result}");
        }
    }
}

fn parse_to_value(input: &str) -> isize {
    if input.ends_with(',') || input.ends_with(':') {
        input[2..input.len() - 1].parse::<isize>().unwrap()
    } else {
        input[2..].parse::<isize>().unwrap()
    }
}
