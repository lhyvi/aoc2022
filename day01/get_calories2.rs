use std::fs;
extern crate itertools;
use itertools::Itertools;

fn main(){
    let input = String::from_utf8(fs::read("input.txt").unwrap()).unwrap();

    let output = input.split("\n\n")
        .map(|s| s.lines().map(|l| l.parse::<usize>().unwrap()).sum::<usize>())
        .sorted()
        .rev()
        .collect::<Vec<_>>();

    print!("p1: {}", output[0]);
    print!("p2: {}", output[0..3].iter().sum());
}
