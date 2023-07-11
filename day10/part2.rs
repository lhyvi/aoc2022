use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let mut cycle = 0;
    let mut register = 1;
    let mut signals_sum = 0;
    for line in input.lines() {
        if let Some((instruction, argument)) = line.split_once(' ') {
            if instruction == "addx" {
                do_cycle(&mut cycle, register, &mut signals_sum);
                do_cycle(&mut cycle, register, &mut signals_sum);
                register += argument.parse::<i32>().unwrap();
            }
        } else {
            let instruction = line;
            if instruction == "noop" {
                do_cycle(&mut cycle, register, &mut signals_sum);
            }
        }
    }
    println!("{}", signals_sum);
}
fn do_cycle(cycle: &mut i32, register: i32, signals: &mut i32) {
    let width = *cycle % 40;
    if width == 0 {
        println!();
    }
    if register >= width - 1 && register <= width + 1 {
        print!("#");
    } else {
        print!(".");
    }
    *cycle += 1;
}
