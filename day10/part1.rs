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
    *cycle += 1;
    if *cycle % 40 == 20 && *cycle <= 220 {
        *signals += *cycle * register;
    }
}
