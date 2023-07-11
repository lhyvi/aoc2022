use std::fs::read_to_string;
use std::collections::HashSet;
fn main() {
    let input = read_to_string("input.txt").unwrap();
    println!("{} {}", solve(&input, 4), solve(&input, 14));
}
fn solve(input: &String, len: usize) -> usize {
    'o: for window in input.chars().enumerate().collect::<Vec<_>>().windows(len) {
        let mut letters: HashSet<char> = HashSet::new();

        for letter in window {
            if !letters.insert(letter.1) {
                continue 'o;
            }
        }

        return window[len - 1].0 + 1;
    }
    return 0;
}
