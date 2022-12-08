use std::fs;
use std::ops::Range;
use std::path::Path;

fn main() {
    let path = Path::new("./src/bin/inputs/day8-input.txt");
    let contents = fs::read_to_string(path)
        .expect("Could not read the file");
    
    let grid = contents
        .trim()
        .lines()
        .map(|line| line
            .chars()
            .map(|char| char.to_digit(10).unwrap())
            .collect::<Vec<u32>>())
        .collect::<Vec<_>>();

    let mut scores = Vec::<i32>::new();
    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            if row == 0 || col == 0 {
                continue
            }
            
            // up, down, left, right
            let directions = vec![
                Direction::Up(0..row),
                Direction::Down(row+1..grid.len()),
                Direction::Left(0..col),
                Direction::Right(col+1..grid[0].len()),
            ];

            let curr_height = grid[row][col];
            
            let mut score = 1;
            for direction in directions {
                let mut count_in_direction = 0;
                if let Direction::Up(range) = direction {
                    for index in range.rev() {
                        if curr_height > grid[index][col] {
                            count_in_direction += 1;
                        } else {
                            count_in_direction += 1;
                            break
                        }
                    }
                } else if let Direction::Down(range) = direction  {
                    for index in range {
                        if curr_height > grid[index][col] {
                            count_in_direction += 1;
                        } else {
                            count_in_direction += 1;
                            break
                        }
                    }
                } else if let Direction::Left(range) = direction  {
                    for index in range.rev() {
                        if curr_height > grid[row][index] {
                            count_in_direction += 1;
                        } else {
                            count_in_direction += 1;
                            break
                        }
                    }
                } else if let Direction::Right(range) = direction  {
                    for index in range {
                        if curr_height > grid[row][index] {
                            count_in_direction += 1;
                        } else {
                            count_in_direction += 1;
                            break
                        }
                    }
                }
                
                score *= count_in_direction;
            }

            scores.push(score);
        }
    }
    
    let highest_score = scores.iter().max().unwrap();
    println!("{} highest score", highest_score);
}

#[derive(Debug)]
enum Direction {
    Up(Range<usize>),
    Down(Range<usize>),
    Left(Range<usize>),
    Right(Range<usize>),
}
