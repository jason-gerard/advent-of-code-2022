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
    
    let common_denominator = monkeys
        .iter()
        .map(|monkey| monkey.test.0)
        .product::<u64>();
    
    let mut inspections = HashMap::<usize, u64>::new();
    
    let rounds = 10000;
    for round in 0..rounds {
        let mut i = 0;
        for monkey in monkeys.clone() {
            for item in monkeys[i].items.clone().into_iter() {
                *inspections.entry(i).or_insert(0) += 1;
                
                let mut new_item = match monkey.operation {
                    Operation::Mult(val) => if val == 0 { item * item } else { item * val },
                    Operation::Add(val) => if val == 0 { item + item } else { item + val },
                };
                
                new_item = new_item % common_denominator;
                
                match new_item % monkey.test.0 == 0 {
                    true => monkeys[monkey.test.1 as usize].items.push(new_item),
                    false => monkeys[monkey.test.2 as usize].items.push(new_item),
                };
                
                monkeys[i].items.remove(0);
            }
            i += 1;
        }
        
        // println!("===After round {} ===", round + 1);
        // dbg!(&inspections);
        // println!();
    }
    
    let mut monkey_business = inspections.values().collect::<Vec<_>>();
    monkey_business.sort_by(|a, b| b.cmp(a));
    let sum = monkey_business[0] * monkey_business[1];
    println!("{} level of monkey business", sum);
}

fn parse_monkey(section: &str) -> Monkey {
    let mut lines = section.lines().skip(1);
    
    let items = lines
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap().1.split(", ")
        .map(|val| val.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    
    let operation_line = lines.next().unwrap();
    let val = if &operation_line[25..] != "old" { *&operation_line[25..].parse::<u64>().unwrap() } else { 0 };
    let operator = operation_line.chars().nth(23).unwrap();
    let operation = match operator { 
        '*' => Operation::Mult(val),
        '+' => Operation::Add(val),
        _ => panic!("invalid operator passed"),
    };
    
    let div_by = *&lines.next().unwrap()[21..].parse::<u64>().unwrap();
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
    items: Vec<u64>,
    operation: Operation,
    test: (u64, u32, u32),
}

#[derive(Debug, Clone)]
enum Operation {
    Add(u64),
    Mult(u64),
}
