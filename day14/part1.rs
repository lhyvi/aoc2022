use std::collections::HashMap;
use std::fs;
use std::ops::Add;
use std::ops::Sub;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut cave = Cave::parse(&input);
    let mut count = 0;
    while cave.drop_sand() {
        count += 1;
    }
    cave.print();
    println!("cave sand: {:?}", count);
}

#[derive(Debug, PartialEq)]
enum Element {
    Rock,
    Sand,
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(num1: i32, num2: i32) -> Self {
        Self { x: num1, y: num2 }
    }
    fn parse_point(input: &str) -> Self {
        //"123,4"
        let (num1, num2) = if let Some((num1, num2)) = input.split_once(",") {
            (num1, num2)
        } else {
            ("0", "0")
        };
        Point {
            x: num1.parse().unwrap(),
            y: num2.parse().unwrap(),
        }
    }
    fn abs_max(&self) -> i32 {
        if self.x.abs() > self.y.abs() {
            self.x.abs()
        } else {
            self.y.abs()
        }
    }
    fn flatten(&self) -> Self {
        Self {
            x: if self.x > 0 {
                1
            } else if self.x < 0 {
                -1
            } else {
                0
            },
            y: if self.y > 0 {
                1
            } else if self.y < 0 {
                -1
            } else {
                0
            },
        }
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

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

#[derive(Debug)]
struct Cave {
    cave: HashMap<Point, Element>,
    deepest: i32,
    min_x: i32,
    max_x: i32,
}

impl Cave {
    fn print(&self) {
        for y in 0..=self.deepest {
            for x in self.min_x..=self.max_x {
                let current_point = Point::new(x, y);
                match self.cave.get(&current_point) {
                    Some(Element::Sand) => print!("o"),
                    Some(Element::Rock) => print!("#"),
                    None => print!("."),
                }
            }
            println!();
        }
    }
    fn parse(input: &str) -> Self {
        let mut cave = Cave {
            cave: HashMap::new(),
            deepest: 0,
            min_x: 500,
            max_x: 500,
        };
        for line in input.lines() {
            let path: Vec<Point> = line.split(" -> ").map(|x| Point::parse_point(x)).collect();
            let mut prev_point = path[0];
            if prev_point.y > cave.deepest {
                cave.deepest = prev_point.y;
            }
            if cave.min_x > prev_point.x {
                cave.min_x = prev_point.x;
            }
            if cave.max_x < prev_point.x {
                cave.max_x = prev_point.x;
            }
            cave.cave.insert(prev_point, Element::Rock);
            for index in 1..path.len() {
                let current_point = path[index];
                if current_point.y > cave.deepest {
                    cave.deepest = current_point.y;
                }
                if cave.min_x > current_point.x {
                    cave.min_x = current_point.x;
                }
                if cave.max_x < current_point.x {
                    cave.max_x = current_point.x;
                }
                let difference = prev_point - current_point;
                for _ in 0..difference.abs_max() {
                    prev_point = prev_point - difference.flatten();
                    cave.cave.insert(prev_point, Element::Rock);
                }
            }
        }
        cave
    }
    fn drop_sand(&mut self) -> bool {
        let mut sand = Point::new(500, 0);
        loop {
            if sand.y > self.deepest {
                return false;
            }
            let bottom = self.cave.get(&(sand + DOWN));
            let bottom_left = self.cave.get(&(sand + DOWN_LEFT));
            let bottom_right = self.cave.get(&(sand + DOWN_RIGHT));
            if bottom == None {
                sand = sand + DOWN;
                continue;
            } else if bottom_left == None {
                sand = sand + DOWN_LEFT;
                continue;
            } else if bottom_right == None {
                sand = sand + DOWN_RIGHT;
                continue;
            } else {
                self.cave.insert(sand, Element::Sand);
                break;
            }
        }
        return true;
    }
}
const DOWN: Point = Point { x: 0, y: 1 };
const DOWN_LEFT: Point = Point { x: -1, y: 1 };
const DOWN_RIGHT: Point = Point { x: 1, y: 1 };
