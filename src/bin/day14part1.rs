use std::cmp::{max, min};
use std::fs;
use std::path::Path;

fn main () {
    let path = Path::new("./src/bin/inputs/day14-input.txt");
    let contents = fs::read_to_string(path)
        .expect("Could not read the file");
    
    let paths = contents
        .trim()
        .lines()
        .map(|line| line.split(" -> ").collect::<Vec<_>>())
        .collect::<Vec<_>>();
    
    let max_height = contents
        .trim()
        .lines()
        .flat_map(|line| line.split(" -> ").map(|point| point.split_once(",").unwrap().1.parse::<u32>().unwrap()))
        .max()
        .unwrap();

    let max_width = contents
        .trim()
        .lines()
        .flat_map(|line| line.split(" -> ").map(|point| point.split_once(",").unwrap().0.parse::<u32>().unwrap()))
        .max()
        .unwrap();
    
    let min_width = contents
        .trim()
        .lines()
        .flat_map(|line| line.split(" -> ").map(|point| point.split_once(",").unwrap().0.parse::<u32>().unwrap()))
        .min()
        .unwrap();
    
    let mut grid = Vec::<Vec<Material>>::new();
    for i in 0..max_height+2 {
        let mut row = Vec::<Material>::new();
        for j in min_width..max_width+3 {
            row.push(Material::Air);
        }
        
        grid.push(row);
    }

    for group in paths.iter() {
        for path in group.windows(2) {
            let (start_col, start_row) = path[0].split_once(",").unwrap();
            let (end_col, end_row) = path[1].split_once(",").unwrap();
            
            let mut start_col = (start_col.parse::<u32>().unwrap() - min_width) as usize;
            let mut start_row = start_row.parse::<usize>().unwrap();
            let mut end_col = (end_col.parse::<u32>().unwrap() - min_width) as usize;
            let mut end_row = end_row.parse::<usize>().unwrap();
            
            if start_col > end_col {
                swap(&mut start_col, &mut end_col);
            }
            if start_row > end_row {
                swap(&mut start_row, &mut end_row);
            }

            for i in start_row..end_row+1 {
                for j in start_col..end_col+1 {
                    grid[i][j+1] = Material::Rock;
                }
            }
        }
    }
    
    print_grid(&grid, min_width);
    
    let mut units = 0;
    'outer: loop {
        let mut curr_col = (500 - min_width) as usize + 1;
        let mut curr_row: usize = 1;
        loop {
            if curr_row == max_height as usize {
                break 'outer
            }
            if grid[curr_row+1][curr_col] == Material::Air {
                curr_row += 1;
                continue
            }
            if grid[curr_row+1][curr_col-1] == Material::Air {
                curr_row += 1;
                curr_col -= 1;
                continue
            }
            if grid[curr_row+1][curr_col+1] == Material::Air {
                curr_row += 1;
                curr_col += 1;
                continue
            }
            
            break
        }
        
        grid[curr_row][curr_col] = Material::Sand;
        units += 1;
    }
    
    print_grid(&grid, min_width);
    println!("{} units of sand", units);
}

fn print_grid(grid: &Vec<Vec<Material>>, min_width: u32) {
    for i in 0..grid.len() {
        for j in 0.. grid[i].len() {
            match (i, j) {
                (0, j) if 500 - min_width as usize + 1 == j => print!("+"),
                (_, _) if grid[i][j] == Material::Air => print!("."),
                (_, _) if grid[i][j] == Material::Rock => print!("#"),
                (_, _) if grid[i][j] == Material::Sand => print!("o"),
                (_, _) => panic!("Invalid value"),
            }
        }
        println!();
    }
}

fn swap(a: &mut usize, b: &mut usize) {
    let t: usize = *a;
    *a = *b;
    *b = t;
}

#[derive(Debug, PartialEq)]
enum Material {
    Air,
    Rock,
    Sand,
}
