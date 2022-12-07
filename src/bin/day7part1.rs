use std::collections::HashSet;
use std::fs;
use std::path::Path;

fn main() {
    let path = Path::new("./src/bin/inputs/day7-input-test.txt");
    let contents = fs::read_to_string(path)
        .expect("Could not read the file");

    println!("{}", contents);
    
    // split input into commands
    // $ cd dir_name
    // $ cd ..
    // $ ls Vec<file>
    let commands: Vec<Command> = vec![];
    let inputs = contents
        .trim()
        .split("$")
        .map(|command_input| command_input.trim())
        .skip(1);
    
    let commands = inputs
        .map(|input| match input { 
            input if input.starts_with("cd ..") => Command::CdUp,
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
    
    dbg!(commands);
    
    // create a stack that is your current path
    // create hash map from stack.to_string -> Vec<file>
    // loop through list of commands and match on type
    // cd up -> pop from stack
    // cd dir_name -> push to stack
    // ls -> insert list of files into hash map
    
    // create hash map to store paths -> size
    // for each path in paths
    // path = a/b/c
    // size = get total size of files form path
    // split path into permutations
    // [/a, /a/b, /a/b/c]
    // for perm in permutations
    // increment hashmap at perm by size
    
    // map hash map to size
    // filter sizes < 100000
    // sum sizes
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
