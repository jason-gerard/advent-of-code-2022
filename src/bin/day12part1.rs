use std::collections::VecDeque;
use std::fs;
use std::path::Path;

fn main() {
    let path = Path::new("./src/bin/inputs/day12-input.txt");
    let contents = fs::read_to_string(path)
        .expect("Could not read the file");
    
    let mut map = Vec::<Vec<u32>>::new();
    let mut start_point = Point {
        row: 0,
        col: 0,
        height: 0,
    };
    let mut end_point = Point {
        row: 0,
        col: 0,
        height: 0,
    };

    let lines = contents.trim().lines().collect::<Vec<_>>();
    for row in 0..lines.len() {
        let chars = lines[row].chars().collect::<Vec<_>>();
        let mut map_row = Vec::<u32>::new();
        for col in 0..lines[row].len() {
            let height = match chars[col] {
                'S' => {
                    start_point = Point {
                        row,
                        col,
                        height: 'a' as u32 - 96,
                    };
                    'a' as u32
                },
                'E' => {
                    end_point = Point {
                        row,
                        col,
                        height: 'z' as u32 - 96,
                    };
                    'z' as u32
                },
                _ => {
                    chars[col] as u32
                },
            };
            
            map_row.push(height - 96);
        }
        map.push(map_row);
    }
    
    let mut queue = VecDeque::<Vec<Point>>::new();
    queue.push_back(vec![start_point.clone()]);
    
    // path length when point was visited
    let mut visited = Vec::<(Point, usize)>::new();
    
    let mut paths = Vec::<Vec<Point>>::new();
    
    while !queue.is_empty() {
        let mut path = queue.pop_front().unwrap();
        let curr_point = path.pop().unwrap();
        
        if curr_point == end_point {
            path.push(curr_point);
            paths.push(path);
            continue
        }
        
        if let Some((_, len)) = visited.iter().find(|&(point, _)| point == &curr_point) {
            if path.len()+1 >= *len {
                continue
            } else {
                let index = visited.iter().position(|(point, _)| point == &curr_point).unwrap();
                visited.remove(index);
                visited.push((curr_point.clone(), path.len() + 1));
            }
        } else {
            visited.push((curr_point.clone(), path.len()+1));
        }

        if curr_point.col+1 < map[0].len() && curr_point.height+1 >= map[curr_point.row][curr_point.col+1] {
            let mut new_path = Vec::<Point>::new();
            for point in path.clone() {
                new_path.push(point.clone());
            }
            new_path.push(curr_point.clone());
            let new_point = Point {
                row: curr_point.row,
                col: curr_point.col+1,
                height: map[curr_point.row][curr_point.col+1],
            };
            new_path.push(new_point.clone());

            queue.push_back(new_path);
        }
        if curr_point.row >= 1 && curr_point.height+1 >= map[curr_point.row-1][curr_point.col] {
            let mut new_path = Vec::<Point>::new();
            for point in path.clone() {
                new_path.push(point.clone());
            }
            new_path.push(curr_point.clone());
            let new_point = Point {
                row: curr_point.row-1,
                col: curr_point.col,
                height: map[curr_point.row-1][curr_point.col],
            };
            new_path.push(new_point.clone());

            queue.push_back(new_path);
        }
        if curr_point.row+1 < map.len() && curr_point.height+1 >= map[curr_point.row+1][curr_point.col] {
            let mut new_path = Vec::<Point>::new();
            for point in path.clone() {
                new_path.push(point.clone());
            }
            new_path.push(curr_point.clone());
            let new_point = Point {
                row: curr_point.row+1,
                col: curr_point.col,
                height: map[curr_point.row+1][curr_point.col],
            };
            new_path.push(new_point.clone());

            queue.push_back(new_path);
        }
        if curr_point.col >= 1 && curr_point.height+1 >= map[curr_point.row][curr_point.col-1] {
            let mut new_path = Vec::<Point>::new();
            for point in path.clone() {
                new_path.push(point.clone());
            }
            new_path.push(curr_point.clone());
            let new_point = Point {
                row: curr_point.row,
                col: curr_point.col-1,
                height: map[curr_point.row][curr_point.col-1],
            };
            new_path.push(new_point.clone());

            queue.push_back(new_path);
        }
    }
    
    dbg!(&paths);
    
    let min = paths
        .iter()
        .map(|path| path.len())
        .min()
        .unwrap();
    
    println!("{} number of steps", min - 1);
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Point {
    row: usize,
    col: usize,
    height: u32,
}
