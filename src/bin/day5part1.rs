use std::collections::VecDeque;
use std::{fmt, fs};
use std::path::Path;

fn main() {
    let path = Path::new("./src/bin/inputs/day5-input.txt");
    let contents = fs::read_to_string(path)
        .expect("Could not read the file");

    let mut stacks: Vec<VecDeque<char>> = vec![];
    let (initial, moves_input) = contents.split_once("\n\n").unwrap();
    
    let first_line = initial.clone().lines().next().unwrap().chars().collect::<Vec<_>>();
    // len = num of stacks * 3 + num of stacks - 1
    // len = num of stacks (3+1) - 1
    // floor(len / 4) + 1 = num of stacks
    let num_stacks = ((first_line.len() as f32 / 4.0).floor() + 1.0) as u32;
    for _ in 0..num_stacks {
        let deque: VecDeque<char> = VecDeque::new();
        stacks.push(deque);
    }

    for line in initial.lines() {
        let chars = line.chars().collect::<Vec<_>>();
        if !chars.contains(&'[') {
            continue
        }
        
        // start at index 1 then add 4 num_stacks - 1 times
        // if letter push to front of the stack
        // else skip
        for i in (1..(4 * (num_stacks - 1) + 2)).step_by(4) {
            if chars[i as usize] != ' ' {
                let index = (i - 1) / 4;
                stacks[index as usize].push_front(chars[i as usize]);
            }
        }
    }
    
    let moves = moves_input
        .trim()
        .lines()
        .map(|line| {
            let parts = line.split(" ").collect::<Vec<&str>>();
            Move {
                number: parts[1].parse().unwrap(),
                from: parts[3].parse::<usize>().unwrap() - 1,
                to: parts[5].parse::<usize>().unwrap() - 1,
            }
        });
    
    moves.for_each(|move_input| {
        for _ in 0..move_input.number {
            let item = stacks[move_input.from].pop_back().unwrap();
            stacks[move_input.to].push_back(item);
        }
    });
    
    // let mut top_crates = "".to_string();
    // for mut stack in stacks {
    //     let top_crate = stack.pop_back().unwrap();
    //     top_crates += &top_crate.to_string();
    // }
    
    let top_crates = stacks
        .into_iter()
        .fold("".to_string(), |curr, mut stack| curr + &stack.pop_back().unwrap().to_string());
    
    // let top_crates = stacks
    //     .into_iter()
    //     .map(|mut stack| stack.pop_back().unwrap().to_string())
    //     .collect::<Vec<_>>()
    //     .concat();
    
    println!("{} top crates", top_crates);
}

struct Move {
    number: u32,
    from: usize,
    to: usize,
}

impl fmt::Display for Move {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str("Number: ");
        fmt.write_str(&self.number.to_string());
        fmt.write_str(" From: ");
        fmt.write_str(&self.from.to_string());
        fmt.write_str(" To: ");
        fmt.write_str(&self.to.to_string());
        Ok(())
    }
}
