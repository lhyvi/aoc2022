use std::fs;

fn main(){
    let input = String::from_utf8(fs::read("input.txt").unwrap()).unwrap();
    let (mut max_calories, mut current_calories) = (0, 0);

    for line in input.lines() {
        if line == "" {
            current_calories = 0;
        } else {
            current_calories = current_calories + line.parse::<i32>().unwrap();
            if current_calories > max_calories {
                max_calories = current_calories;
            }
        }
    }
    print!("{}", max_calories);
}
