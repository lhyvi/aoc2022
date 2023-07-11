use std::fs;

fn main(){
    let input = String::from_utf8(fs::read("input.txt").unwrap()).unwrap();
    let lose = 0;
    let draw = 3;
    let win = 6;
    let mut score = 0;

    for line in input.lines()
    {
        let lxne: Vec<char> = line.chars().collect();
        let elf = lxne[0] as i32 - 'A' as i32 + 1;
        let player = lxne[2] as i32 - 'X' as i32 + 1;
        let result = elf - player;
        if result == 0 {
            score += draw + player;
        } else if result == -1 || result == 2 {
            score += win + player;
        } else {
            score += lose + player;
        }
    }
    println!("{}", score);
}
