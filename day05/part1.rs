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
            .filter_map(|(i, e)| ((i + 1) % 2 == 0).then(|| e.parse::<i32>().unwrap()))
            .collect();
        for _ in 0..values[0] {
            if let Some(ch) = stacks[values[1] as usize - 1].pop(){
                stacks[values[2] as usize - 1].push(ch);
            }
        }
    }

    for stack in stacks.iter() {
        print!("{}", stack.last().unwrap_or(&"".to_string()));
    }
}
