use std::collections::HashMap;
use std::fs;
use std::path::Path;

fn main() {
    let path = Path::new("./src/bin/inputs/day11-input.txt");
    let contents = fs::read_to_string(path)
        .expect("Could not read the file");
    
    let mut monkeys = contents
        .trim()
        .split("\n\n")
        .map(parse_monkey)
        .collect::<Vec<_>>();
    
    let mut inspections = HashMap::<usize, u32>::new();
    
    let rounds = 20;
    for _ in 0..rounds {
        let mut i = 0;
        for monkey in monkeys.clone() {
            for item in monkeys[i].items.clone().into_iter() {
                *inspections.entry(i).or_insert(0) += 1;
                
                let mut new_item = match monkey.operation {
                    Operation::Mult(val) => if val == 0 { item * item } else { item * val },
                    Operation::Add(val) => if val == 0 { item + item } else { item + val },
                };
                new_item /= 3;
                
                match new_item % monkey.test.0 == 0 {
                    true => monkeys[monkey.test.1 as usize].items.push(new_item),
                    false => monkeys[monkey.test.2 as usize].items.push(new_item),
                };
                
                monkeys[i].items.remove(0);
            }
            i += 1;
        }
    }
    
    let mut monkey_business = inspections.values().collect::<Vec<_>>();
    monkey_business.sort_by(|a, b| b.cmp(a));
    println!("{} level of monkey business", monkey_business[0] * monkey_business[1]);
}

fn parse_monkey(section: &str) -> Monkey {
    let mut lines = section.lines().skip(1);
    
    let items = lines
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap().1.split(", ")
        .map(|val| val.parse::<u32>().unwrap())
        .collect::<Vec<_>>();
    
    let operation_line = lines.next().unwrap();
    let val = if &operation_line[25..] != "old" { *&operation_line[25..].parse::<u32>().unwrap() } else { 0 };
    let operator = operation_line.chars().nth(23).unwrap();
    let operation = match operator { 
        '*' => Operation::Mult(val),
        '+' => Operation::Add(val),
        _ => panic!("invalid operator passed"),
    };
    
    let div_by = *&lines.next().unwrap()[21..].parse::<u32>().unwrap();
    let true_test = *&lines.next().unwrap()[29..].parse::<u32>().unwrap();
    let false_test = *&lines.next().unwrap()[30..].parse::<u32>().unwrap();
    
    return Monkey {
        items,
        operation,
        test: (div_by, true_test, false_test),
    };
}

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<u32>,
    operation: Operation,
    test: (u32, u32, u32),
}

#[derive(Debug, Clone)]
enum Operation {
    Add(u32),
    Mult(u32),
}
