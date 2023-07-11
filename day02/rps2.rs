use std::fs;

// rps
// 1 2 3
// win
// 0 - 1
// 1 - 2
// 2 - 0
// lose
// 0 - 2
// 1 - 0
// 2 - 1
fn main(){
    let input = String::from_utf8(fs::read("input.txt").unwrap()).unwrap();
    let lose = 0;
    let draw = 3;
    let win = 6;
    let mut score = 0;

    for line in input.lines()
    {
        let lxne: Vec<char> = line.chars().collect();
        let mut elf = lxne[0] as i32 - 'A' as i32 + 1;
        let player = lxne[2] as i32 - 'X' as i32;
        let result = player;
        if player == 0 {
            elf -= 1;
            if elf == 0 {
                elf = 3;
            }
            score += lose + elf;
        } else if player == 1 {
            score += draw + elf;
        } else {
            elf += 1;
            if elf == 4 {
                elf = 1;
            }
            score += win + elf;
        }
    }
    println!("{}", score);
}
