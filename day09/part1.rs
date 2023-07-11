use std::collections::HashSet;
use std::fs::read_to_string;
use std::ops::{Sub, Add};

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let mut head = Point {x: 0, y: 0};
    let mut tail = Point {x: 0, y: 0};
    let mut visited: HashSet<Point> = HashSet::new();

    for line in input.lines() {
        let (direction, distance) = line.split_once(' ').unwrap();
        let distance = distance.parse::<i32>().unwrap();
        visited.insert(tail);
        for _ in 0..distance {
            head.move_point(dir_to_point(direction));
            tail.follow(head);
            visited.insert(tail);
        }
    }
    println!("{}", visited.len());
}

const UP: Point = Point {x: 0, y: 1};
const DOWN: Point = Point {x: 0, y: -1};
const LEFT: Point = Point {x: -1, y: 0};
const RIGHT: Point = Point {x: 1, y: 0};

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}
impl Point {
    fn move_point(&mut self, other: Self) {
        self.x = self.x + other.x;
        self.y = self.y + other.y;
    }
    fn follow(&mut self, head: Self) {
        let distance = head - *self;
        if distance.x.abs() == 2{
            self.x += distance.x / 2;
            self.y += distance.y;
        } else if distance.y.abs() == 2{
            self.x += distance.x;
            self.y += distance.y / 2;
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

fn dir_to_point(dir: &str) -> Point {
    match dir {
        "R" => RIGHT,
        "L" => LEFT,
        "D" => DOWN,
        "U" => UP,
        _ => Point {x: 0, y: 0},
    }
}

