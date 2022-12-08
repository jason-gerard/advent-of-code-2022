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

    let mut visible_trees_count = 0;
    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            if row == 0 || col == 0 {
                visible_trees_count += 1;
                continue
            }
            
            // up, down, left, right
            let directions = vec![
                Direction::Vertical(0..row), 
                Direction::Vertical(row+1..grid.len()),
                Direction::Horizontal(0..col),
                Direction::Horizontal(col+1..grid[0].len()),
            ];

            let curr_height = grid[row][col];
            let directions_visible = directions
                .into_iter()
                .map(|direction| match direction {
                    Direction::Vertical(mut range) => range.all(|index| curr_height > grid[index][col]),
                    Direction::Horizontal(mut range) => range.all(|index| curr_height > grid[row][index]),
                })
                .any(|is_direction_visible| is_direction_visible);
            
            if directions_visible {
                visible_trees_count += 1;
            }
        }
    }
    
    println!("{} number of visible trees", visible_trees_count);
}

#[derive(Debug)]
enum Direction {
    Vertical(Range<usize>),
    Horizontal(Range<usize>),
}
