use std::collections::VecDeque;
use std::fs;
use std::path::Path;

fn main() {
    let path = Path::new("./src/bin/day5-input.txt");
    let contents = fs::read_to_string(path)
        .expect("Could not read the file");

    let mut stacks: Vec<VecDeque<char>> = vec![];
    let (initial, moves) = contents.split_once("\n\n").unwrap();
    
    let first_line = initial.clone().lines().next().unwrap().chars().collect::<Vec<_>>();
    // len = num of stacks * 3 + num of stacks - 1
    // len = num of stacks (3+1) - 1
    // floor(len / 4) + 1 = num of stacks
    let num_stacks = ((first_line.len() as f32 / 4.0).floor() + 1.0) as u32;
    for _ in 0..num_stacks {
        let mut deque: VecDeque<char> = VecDeque::new();
        stacks.push(deque);
    }

    for line in initial.lines() {
        let chars = line.chars().collect::<Vec<_>>();
        if chars[0] == ' ' {
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
        println!("{}", line);
        println!("{} -> {}", chars.len(), num_stacks);
    }
    
    println!("{:?}", stacks);
}
