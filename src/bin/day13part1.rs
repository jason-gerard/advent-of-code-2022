use std::collections::VecDeque;
use std::fs;
use std::path::Path;

fn main() {
    let path = Path::new("./src/bin/inputs/day13-input.txt");
    let contents = fs::read_to_string(path)
        .expect("Could not read the file");
    
    let contents = contents.replace("10", "A");
    let num_valid_orders = contents
        .trim()
        .split("\n\n")
        .enumerate()
        .map(|(index, line)| {
            let (left_input, right_input) = line.split_once("\n").unwrap();
            return (index as u32 + 1) * check_order(&*left_input.chars().collect::<Vec<_>>(), &*right_input.chars().collect::<Vec<_>>());
        })
        .sum::<u32>();

    println!("sum of valid orders {}", num_valid_orders);
}

fn check_order(left: &[char], right: &[char]) -> u32 {
    return match (left[0], right[0]) {
        (l, r) if l == r => check_order(&left[1..], &right[1..]),
        (_, ']') => 0,
        (']', _) => 1,
        (_, '[') => {
            let converted_left = &*[&[left[0], ']'], &left[1..]].concat::<char>();
            return check_order(converted_left, &right[1..]);
        },
        ('[', _) => {
            let converted_right = &*[&[right[0], ']'], &right[1..]].concat::<char>();
            return check_order(&left[1..], converted_right);
        },
        (l, r) => if l <= r { 1 } else { 0 },
    };
}

// enum Packet {
//     Int(u32),
//     List(Vec<Packet>),
// }
