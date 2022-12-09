use std::collections::HashSet;
use std::fs;
use std::path::Path;

fn main() {
    let path = Path::new("./src/bin/inputs/day9-input.txt");
    let contents = fs::read_to_string(path)
        .expect("Could not read the file");
    
    // create set of points where tail visited
    let mut visited = HashSet::<Point>::new();
    visited.insert(Point {
        row: 0,
        col: 0,
    });
    // tail is index 9
    let mut rope = Vec::<Point>::new();
    for _ in 0..10 {
        rope.push(Point {
            row: 0,
            col: 0,
        });
    }

    
    let moves = contents
        .trim()
        .lines()
        .map(|line| {
            let (direction_input, repetition_input) = line.split_once(" ").unwrap();
            let direction = match direction_input {
                "U" => Direction::Up,
                "D" => Direction::Down,
                "L" => Direction::Left,
                "R" => Direction::Right,
                _ => panic!("Invalid direction")
            };
            
            return Move {
                direction,
                repetition: repetition_input.parse::<i32>().unwrap(),
            };
        });
    
    for movement in moves {
        for i in 0..movement.repetition {
            let mut old_rope = Vec::<Point>::new();
            for i in 0..10 {
                old_rope.push(Point {
                    row: rope[i].row,
                    col: rope[i].col,
                });
            }
            let mut col_diff = 0;
            let mut row_diff = 0;
            
            for j in 0..rope.len() - 1 {
                // update index of head
                if j == 0 {
                    match movement.direction {
                        Direction::Up => rope[j].row -= 1,
                        Direction::Down => rope[j].row += 1,
                        Direction::Left => rope[j].col -= 1,
                        Direction::Right => rope[j].col += 1,
                    }
                    print_grid(&rope);
                }
                if !is_adjacent(&rope[j], &rope[j+1]) {
                    if (rope[j].col - rope[j+1].col).abs() > 1 && (rope[j].row - rope[j+1].row).abs() > 1 {
                        rope[j+1].col = if rope[j].col > rope[j+1].col {rope[j+1].col + 1} else {rope[j+1].col - 1};
                        rope[j+1].row = if rope[j].row > rope[j+1].row {rope[j+1].row + 1} else {rope[j+1].row - 1};
                    } else {
                        if rope[j].col - rope[j+1].col < -1 {
                            rope[j+1].row = rope[j].row;
                            rope[j+1].col = rope[j].col + 1;
                        } else if rope[j].col - rope[j+1].col > 1 {
                            rope[j+1].row = rope[j].row;
                            rope[j+1].col = rope[j].col - 1;
                        } else if rope[j].row - rope[j+1].row < -1 {
                            rope[j+1].col = rope[j].col;
                            rope[j+1].row = rope[j].row + 1;
                        } else if rope[j].row - rope[j+1].row > 1 {
                            rope[j+1].col = rope[j].col;
                            rope[j+1].row = rope[j].row - 1;
                        }
                    }
                    
                    if rope.len()-1 == j+1 {
                        visited.insert(Point {
                            row: rope[j+1].row,
                            col: rope[j+1].col,
                        });
                    }
                    print_grid(&rope);
                }
            }
        }
        // print_grid(&rope);
    }
    
    
    // get sum of set
    let sum = visited.len();
    println!("{} points visited by tail", sum);
}

fn print_grid(rope: &Vec<Point>) {
    for i in 0..40 {
        for j in 0..40 {
            let temp = rope.iter().position(|point| ((point.row)%40).abs() == i && ((point.col)%40).abs() == j);
            if let Some(index) = temp {
                print!("{}", index);
            } else {
                print!(".")
            }
        }
        println!();
    }
    println!();
}

fn is_adjacent(head: &Point, tail: &Point) -> bool {
    return (tail.row + 1 == head.row && tail.col == head.col) ||
        (tail.row - 1 == head.row && tail.col == head.col) ||
        (tail.row == head.row && tail.col + 1 == head.col) ||
        (tail.row == head.row && tail.col - 1 == head.col) ||
        (tail.col + 1 == head.col && tail.row + 1 == head.row) ||
        (tail.col + 1 == head.col && tail.row - 1 == head.row) ||
        (tail.col - 1 == head.col && tail.row + 1 == head.row) ||
        (tail.col - 1 == head.col && tail.row - 1 == head.row) ||
        (tail.row == head.row && tail.col == head.col);
}

fn is_diagonal(head: &Point, tail: &Point) -> bool {
    return (tail.col + 1 == head.col && tail.row + 1 == head.row) ||
        (tail.col + 1 == head.col && tail.row - 1 == head.row) ||
        (tail.col - 1 == head.col && tail.row + 1 == head.row) ||
        (tail.col - 1 == head.col && tail.row - 1 == head.row);
}

#[derive(Debug, Eq, Hash, PartialEq)]
struct Point {
    row: i32,
    col: i32,
}

#[derive(Debug)]
struct Move {
    direction: Direction,
    repetition: i32,
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
