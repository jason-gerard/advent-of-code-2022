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
    // hold index of head
    let mut head = Point {
        row: 0,
        col: 0,
    };
    // hold index of tail
    let mut tail = Point {
        row: 0,
        col: 0,
    };
    
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
            let old_head = Point {
                row: head.row,
                col: head.col,
            };
            // update index of head
            match movement.direction {
                Direction::Up => head.row += 1,
                Direction::Down => head.row -= 1,
                Direction::Left => head.col -= 1,
                Direction::Right => head.col += 1,
            }
            println!("{} ?",  is_adjacent(&head, &tail));
            dbg!(&head, &tail);
            if !is_adjacent(&head, &tail) {
                // update tail and add new point to set
                tail.row = old_head.row;
                tail.col = old_head.col;
                visited.insert(Point {
                    row: tail.row,
                    col: tail.col,
                });
            }
        }
    }
    
    // get sum of set
    dbg!(&visited);
    let sum = visited.len();
    println!("{} points visited by tail", sum);
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
