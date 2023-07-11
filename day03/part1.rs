use std::fs;

fn main(){
    let input = String::from_utf8(fs::read("input.txt").unwrap()).unwrap();
    let mut sum = 0;

    for line in input.lines() {
        let (first, last) = line.split_at(line.len()/2);
        let mut chur_char = 'a';
        for letter in first.chars() {
            if last.contains(letter) {
                chur_char = letter;
                break;
            }
        }
        sum += match chur_char as u32 {
            65..=90 => chur_char as u32 - 'A' as u32 + 27,
            _ => chur_char as u32 - 'a' as u32 + 1,
        };
    }
    println!("{}", sum);
}
