use std::fs;

fn main(){
    let input = String::from_utf8(fs::read("input.txt").unwrap()).unwrap();
    let mut count = 0;
    for line in input.lines() {
        let mut split = line.split(',');
        let elf1: Vec<i32> = split.next().unwrap().split('-').map(|x| x.parse::<i32>().unwrap()).collect();
        let elf2: Vec<i32> = split.next().unwrap().split('-').map(|x| x.parse::<i32>().unwrap()).collect();
        if (elf1[0] >= elf2[0] && elf1[1] <= elf2[1]) || (elf1[0] <= elf2[0] && elf1[1] >= elf2[1]) {
            count += 1;
        }
    }
    println!("{}", count);
}
