use std::fs;

fn main(){
    let input = String::from_utf8(fs::read("input.txt").unwrap()).unwrap();
    let mut sum = 0;
    let lines: Vec<_> = input.lines().collect();
    for three_lines in lines.chunks(3) {
        for character in three_lines[0].chars() {
            if three_lines[1].contains(character) && three_lines[2].contains(character) {
                sum += match character as u32 {
                    65..=90 => character as u32 - 'A' as u32 + 27,
                    _ => character as u32 - 'a' as u32 + 1,
                };
                break;
            }
        }
    }
    println!("{}",sum);

}
