use std::fs;
use std::collections::VecDeque;

fn main() {
    let len = 14;
    let input = String::from_utf8(fs::read("input.txt").unwrap()).unwrap();
    let mut letters: VecDeque<char> = input[0..len].chars().collect::<VecDeque<char>>();
    let mut result = 0;
    let done = true;
    let mut state = true;

    for (i, letter) in input.chars().enumerate() {
        letters.push_back(letter);
        if letters.len() > len {
            letters.pop_front();
        }
        state = false;
        for j in 0..letters.len() {
            for k in j+1..letters.len() {
                if letters[j] == letters[k] {
                    state = done;
                    break;
                }
            }
        }
        if !state {
            result = i + 1;
            break;
        }
    }

    println!("{:?}", result);
}
