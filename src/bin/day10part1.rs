use std::collections::HashSet;
use std::fs;
use std::path::Path;

fn main() {
    let path = Path::new("./src/bin/inputs/day10-input.txt");
    let contents = fs::read_to_string(path)
        .expect("Could not read the file");
    
    let mut cycle = 0;
    let mut register_x = 1;
    let mut signal_strengths = 0;
    
    for instruction in contents.trim().lines() {
        let mut cycles = 1;
        if instruction.starts_with("addx") {
            cycles += 1;
        }
        
        for _ in 0..cycles {
            cycle += 1;
            if (cycle + 20) % 40 == 0 {
                signal_strengths += register_x * cycle;
            }
        }
        
        match instruction {
            add_instruction if instruction.starts_with("addx") => {
                let val = add_instruction.split_once(" ").unwrap().1.parse::<i32>().unwrap();
                register_x += val;
            },
            _ => (),
        };
    }
    
    println!("{} is the sum of signal strengths", signal_strengths);
}
