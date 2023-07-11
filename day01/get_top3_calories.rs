use std::fs;

fn main(){
    let input = String::from_utf8(fs::read("input.txt").unwrap()).unwrap();
    let mut current_calories = 0;
    let mut current_top: [i32; 3] = [0, 0, 0];

    for line in input.lines() {
        if line == "" {
            current_top.sort();
            current_calories = 0;
        } else {
            current_calories = current_calories + line.parse::<i32>().unwrap();
            if current_calories > current_top[0] {
                current_top[0] = current_calories;
            }
        }
    }
    print!("{}", current_top.iter().sum::<i32>());
}
