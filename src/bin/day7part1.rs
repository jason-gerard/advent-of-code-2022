use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;

fn main() {
    let path = Path::new("./src/bin/inputs/day7-input.txt");
    let contents = fs::read_to_string(path)
        .expect("Could not read the file");

    println!("{}", contents);
    
    // split input into commands
    // $ cd dir_name
    // $ cd ..
    // $ ls Vec<file>
    let inputs = contents
        .trim()
        .split("$")
        .map(|command_input| command_input.trim())
        .skip(1);
    
    let commands = inputs
        .map(|input| match input { 
            input if input.starts_with("cd ..") => Command::CdUp,
            input if input.starts_with("cd /") => Command::CdDown("".to_string()),
            input if input.starts_with("cd") => Command::CdDown(input.split_once(" ").unwrap().1.to_string()),
            input if input.starts_with("ls") => {
                let files = input
                    .split("\n")
                    .skip(1)
                    .map(|line| match line {
                        line if line.starts_with("dir") => File::Dir(line.split_once(" ").unwrap().1.to_string()),
                        _ => {
                            let (size, file_name) = line.split_once(" ").unwrap();
                            File::File {
                                name: file_name.to_string(),
                                size: size.parse::<i32>().unwrap(),
                            }
                        }
                    })
                    .collect::<Vec<File>>();
                
                Command::Ls(files)
            },
            _ => panic!("invalid command input"),
        })
        .collect::<Vec<Command>>();
    
    dbg!(commands.iter());
    
    // create a stack that is your current path
    let mut stack: Vec<String> = vec![];
    // create hash map from stack.to_string -> Vec<file>
    let mut paths = HashMap::<String, Vec<File>>::new();
    // loop through list of commands and match on type
    // cd up -> pop from stack
    // cd dir_name -> push to stack
    // ls -> insert list of files into hash map
    for command in commands {
        dbg!(&command);
        if let Command::CdUp = command {
            stack.pop(); 
            continue
        }
        if let Command::Ls(files) = command {
            let path = stack.join("/");
            paths.insert(path.chars().collect::<String>(), files);
            continue
        }
        if let Command::CdDown(dir) = command {
            stack.push(dir);
            continue
        }
    }

    dbg!(&paths);
    
    // create hash map to store paths -> size
    let mut sizes = HashMap::<String, i32>::new();
    // for each path in paths
    for (path, files) in paths {
        // path = a/b/c
        // size = get total size of files form path
        let size = files
            .iter()
            .map(|file| match file {
                File::Dir(_) => 0,
                File::File {size, name } => *size,
            })
            .sum::<i32>();
        
        println!("path {} with size {}", path, size);
        
        let mut path_permutations = vec![""];
        path_permutations.append(&mut path.split("/").filter(|perm| *perm != "").collect::<Vec<&str>>());
        for i in 1..path_permutations.len()+1 {
            // split path into permutations
            // [/a, /a/b, /a/b/c]
            // for perm in permutations
            // increment hashmap at perm by size
            let curr_path = &path_permutations[0..i].join("/");
            println!("{:?}", curr_path);
            *sizes.entry(curr_path.clone()).or_insert(0) += size;
        }
    }
    
    dbg!(&sizes);
    
    // map hash map to size
    // filter sizes < 100000
    // sum sizes
    let total_size = sizes
        .iter()
        .map(|(_, size)| size)
        .filter(|size| size < &&100000)
        .sum::<i32>();
    
    println!("{} total size", total_size);
}

#[derive(Debug)]
enum Command {
    CdUp,
    CdDown(String),
    Ls(Vec<File>),
}

#[derive(Debug)]
enum File {
    Dir(String),
    File {
        name: String,
        size: i32,
    },
}
