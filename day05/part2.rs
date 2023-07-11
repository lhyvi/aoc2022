use std::fs;
fn main() {
    let input: Vec<String> = String::from_utf8(fs::read("input.txt").unwrap()).unwrap().split("\n\n").map(String::from).collect();
    let mut stacks: [Vec<String>; 9] = Default::default();

    for line in input[0].lines() {
        for (i, case) in line.chars().collect::<Vec<char>>().chunks(4).enumerate() {
            if case[1] != ' ' && !case[1].is_digit(10) {
                stacks[i].insert(0, case[1].to_string());
            }
        }
    }

    for line in input[1].lines() {
        let values: Vec<_> = line.split(' ').enumerate()
            .filter_map(|(i, e)| ((i + 1) % 2 == 0).then(|| e.parse::<usize>().unwrap()))
            .collect();
        let len = stacks[values[1] - 1].len();
        let mut top_stack = stacks[values[1] - 1].as_slice()[len - values[0]..].to_vec();
        stacks[values[2] - 1].append(&mut top_stack);
        for _ in 0..values[0] {
            stacks[values[1] - 1].pop();
        }
    }

    for stack in stacks.iter() {
        print!("{}", stack.last().unwrap_or(&"".to_string()));
    }
}
