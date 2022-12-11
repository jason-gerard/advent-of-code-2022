use std::fs;
use std::path::Path;

fn main() {
    let path = Path::new("./src/bin/inputs/day10-input.txt");
    let contents = fs::read_to_string(path)
        .expect("Could not read the file");
    
    let mut cycle = 0;
    let mut register_x = 1;
    
    let mut pixels = Vec::<char>::new();
    
    for instruction in contents.trim().lines() {
        let mut cycles = 1;
        if instruction.starts_with("addx") {
            cycles += 1;
        }
        
        for _ in 0..cycles {
            let pixel = if (register_x-1) <= (cycle % 40) && (cycle % 40) <= (register_x+1) { '#' } else { '.' };
            pixels.push(pixel);
            cycle += 1;
        }
        
        match instruction {
            add_instruction if instruction.starts_with("addx") => {
                let val = add_instruction.split_once(" ").unwrap().1.parse::<i32>().unwrap();
                register_x += val;
            },
            _ => (),
        };
    }
    
    for i in 0..pixels.len() {
        print!("{}", pixels[i]);
        if (i+1) % 40 == 0 {
            println!();
        }
    }
}
