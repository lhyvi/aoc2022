use std::fs::read_to_string;

fn main() {
   let input = read_to_string("input.txt").unwrap();
   let mut curr_monkey = 0;
   let mut monkeys: Vec<Monkey> = vec![Monkey::new()];

   for line in input.lines() {
       if line == "" {
           monkeys.push(Monkey::new());
           curr_monkey += 1;
       } else {
           monkeys[curr_monkey].parse(line);
       }
   }

   for _ in 0..20 {
       for monkey_index in 0..monkeys.len() {
           monkeys[monkey_index].inspect();
           let push_indexes = monkeys[monkey_index].get_push_indexes();
           for (i, index) in push_indexes.iter().enumerate() {
               let item = monkeys[monkey_index].items[i];
               monkeys[*index].items.push(item);
           }
           monkeys[monkey_index].items.clear();
       }
   }
   let mut counts: Vec<usize> = vec![];
   for monkey in monkeys.iter() {
       counts.push(monkey.inspect_count);
   }
   counts.sort();
   let a = counts[counts.len() - 1];
   let b = counts[counts.len() - 2];
   println!("{}", a * b);
}

#[derive(Debug)]
struct Monkey {
    items: Vec<i32>,
    operations: Vec<String>,
    test: i32,
    if_true: usize,
    if_false: usize,
    inspect_count: usize,
}

impl Monkey {
    fn new() -> Self {
        Self {
            items: vec![],
            operations: vec![],
            test: 0,
            if_true: 0,
            if_false: 0,
            inspect_count: 0,
        }
    }
    fn parse(&mut self, line: &str) {
        if let Some(items) = line.strip_prefix("  Starting items: ") {
            for item in items.split(',') {
                self.items.push(item.trim().parse::<i32>().unwrap());
            }
        } else if let Some(operations)  = line.strip_prefix("  Operation: ") {
            for operation in operations.split(' ').rev().take(3) {
                self.operations.push(operation.to_string());
            }
            self.operations.reverse();
        } else if let Some(test) = line.strip_prefix("  Test: ") {
            self.test = test.split(' ').last().unwrap().parse::<i32>().unwrap();
        } else if let Some(if_true) = line.strip_prefix("    If true: throw to monkey ") {
            self.if_true = if_true.parse::<usize>().unwrap();
        } else if let Some(if_false) = line.strip_prefix("    If false: throw to monkey ") {
            self.if_false = if_false.parse::<usize>().unwrap();
        }
    }
    fn inspect(&mut self) {
        for item_index in 0..self.items.len() {
            self.inspect_count += 1;
            let num1 = match self.operations[0].as_str() {
                "old" => self.items[item_index],
                _ => self.operations[0].parse::<i32>().unwrap(),
            };
            let num2 = match self.operations[2].as_str() {
                "old" => self.items[item_index],
                _ => self.operations[2].parse::<i32>().unwrap(),
            };
            self.items[item_index] = match self.operations[1].as_str() {
                "*" => (num1 * num2) / 3,
                "+" => (num1 + num2) / 3,
                _ => 0,
            }
        }
    }
    fn get_push_indexes(&mut self) -> Vec<usize> {
        let mut indexes: Vec<usize> = vec![];
        for item in self.items.iter() {
            let push_index = if item % self.test == 0 {
                self.if_true
            } else {
                self.if_false
            };
            indexes.push(push_index);
        }
        indexes
    }
}
