use std::collections::HashSet;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let point1 = Point::new(1, 2);
    let point2 = Point::new(3, 4);
    let map = Map::parse_input(input);
    let count = map.get_impossible_beacons(2000000);
    println!("{count}");
}

#[derive(Debug, Eq, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}
impl Point {
    fn new(num1: i32, num2: i32) -> Self {
        Point { x: num1, y: num2 }
    }
    fn from_tuple(numbers: (i32, i32)) -> Self {
        Point {
            x: numbers.0,
            y: numbers.1,
        }
    }
    fn get_distance(&self, other: &Self) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
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
    fn get_impossible_beacons(&self, row: i32) -> i32 {
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
}

fn parse_to_value(input: &str) -> i32 {
    if input.ends_with(',') || input.ends_with(':') {
        input[2..input.len() - 1].parse::<i32>().unwrap()
    } else {
        input[2..].parse::<i32>().unwrap()
    }
}
