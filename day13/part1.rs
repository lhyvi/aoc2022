use std::fs::read_to_string;
fn main() {
    let input = read_to_string("input.txt").expect("Can't read file to string");
    let mut count = 0usize;
    for (i, pair) in input.split("\n\n").enumerate() {
        let (left, right) = pair.split_once('\n').unwrap();
        let left_pair = parse_to_pair(left);
        let right_pair = parse_to_pair(right);
        if Pair::compare(&left_pair, &right_pair) {
            count += i + 1;
        }
    }
    println!("{}", count);
}
fn parse_to_pair(input: &str) -> Pair {
    let (values, _) = parse_to_values(&input.chars().rev().collect::<String>());
    let pair = Pair {
        packet: values
    };
    pair
}
fn parse_to_values(input:&str) -> (Vec<Value>, String) {
    let mut list = vec![];
    let mut input = input.to_string();
    let mut num = 0;
    let mut num_exists = false;
    if !input.is_empty() {
        while let Some(current_character) = input.pop() {
            if current_character == ']' {
                if num_exists {
                    list.push( Value::Num(num));
                    num = 0;
                    num_exists = false;
                }
                break;
            } else  if current_character == ',' {
                if num_exists {
                    list.push( Value::Num(num));
                    num = 0;
                    num_exists = false;
                }
            } else if current_character.is_digit(10) {
                num_exists = true;
                num = num * 10 + current_character.to_digit(10).unwrap();
            } else if current_character == '[' {
                let (value, new_input) = parse_to_values(&input);
                input = new_input;
                list.push(Value::List(value));
            }
        }
    }
    (list, input)
}
#[derive(Debug, PartialEq, Clone)]
enum Value {
    List(Vec<Value>),
    Num(u32),
}
#[derive(Debug)]
struct Pair {
    packet: Vec<Value>,
}
impl Pair {
    fn new() -> Self {
        Pair {
            packet: vec![]
        }
    }
    fn compare(left: &Self, right: &Self) -> bool {
        let mut truth = false;
        if let Value::List(left_list) = &left.packet[0] {
            if let Value::List(right_list) = &right.packet[0] {
                truth = compare_lists(left_list.to_vec(), right_list.to_vec()).unwrap();
            }
        }
        truth
    }
}
fn compare_lists(left: Vec<Value>, right: Vec<Value>) -> Option<bool> {
    use Value::{Num, List};
    let mut in_order = true;
    let min = left.len().min(right.len());
    for index in 0..min {
        match (&left[index], &right[index]) {
            (Num(l), Num(r)) => {
                if l == r {
                    continue;
                } else {
                    return Some(l < r);
                }
            },
            (List(l), Num(r)) => {
                let result = compare_lists(l.to_vec(), vec![Num(*r)]);
                if result != None {
                    return result;
                } else {
                    continue;
                }
            },
            (Num(l), List(r)) => {
                let result = compare_lists(vec![Num(*l)], r.to_vec());
                if result != None {
                    return result;
                } else {
                    continue;
                }
            },
            (List(l), List(r)) => {
                let result = compare_lists(l.to_vec(), r.to_vec());
                if result != None {
                    return result;
                } else {
                    continue;
                }
            },
        }
    }
    if left.len() != right.len() {
        return Some(left.len() < right.len());
    }
    return None;
}
