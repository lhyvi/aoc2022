use std::collections::HashSet;
use std::fs::read_to_string;
use std::ops::{Sub, Add};

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let mut rope: Vec<Point> = vec![];
    for _ in 0..10 {
        rope.push(Point {x: 0, y: 0});
    }
    let mut visited: HashSet<Point> = HashSet::new();

    visited.insert(rope[9]);
    for line in input.lines() {
        let (direction, distance) = line.split_once(' ').unwrap();
        let distance = distance.parse::<i32>().unwrap();
        for _ in 0..distance {
            rope[0].move_point(dir_to_point(direction));
            for i in 1..10 {
                let following = rope[i-1];
                rope[i].follow(following);
            }
            visited.insert(rope[9]);
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
        let x_diff = distance.x.abs() == 2;
        let y_diff = distance.y.abs() == 2;
        if x_diff || y_diff {
            if x_diff {
                self.x += distance.x / 2;
            } else {
                self.x += distance.x;
            }
            if y_diff {
                self.y += distance.y / 2;
            } else {
                self.y += distance.y;
            }
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

fn visualize(rope: &Vec<Point>) {
    let len = 20;
    for i in (-len..len).rev() {
        print!("{}", i);
        for j in -len..len {
            let k = Point {x: j, y: i};
            let mut result = 0;
            for l in (0..10).rev() {
                if k == rope[l] {
                    result = l + 1;
                }
            }
            if result == 1 {
                print!("H");
            } else if result == 0{
                print!(".");
            } else {
                print!("{}", (result - 1).to_string());
            }
        }
        println!();
    }
    println!();
}
