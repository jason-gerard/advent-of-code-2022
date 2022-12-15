use std::cmp::Ordering;
use std::collections::VecDeque;
use std::fs;
use std::path::Path;

fn main() {
    let path = Path::new("./src/bin/inputs/day13-input.txt");
    let contents = fs::read_to_string(path)
        .expect("Could not read the file");
    
    let contents = contents.replace("10", "A");
    let contents = contents.replace("\n\n", "\n");
    
    let mut packets = contents
        .trim()
        .lines()
        .collect::<Vec<_>>();
    packets.push("[[2]]");
    packets.push("[[6]]");
    
    packets.sort_by(|a, b| check_order(&*a.chars().collect::<Vec<_>>(), &*b.chars().collect::<Vec<_>>()));
    packets.reverse();
    
    for packet in packets.iter() {
        println!("{}", packet);
    }
    
    let div_1 = packets.iter().position(|p| p == &"[[2]]").unwrap() + 1;
    let div_2 = packets.iter().position(|p| p == &"[[6]]").unwrap() + 1;
    
    println!("decoder key: {}", div_1 * div_2);
}

fn check_order(left: &[char], right: &[char]) -> Ordering {
    return match (left[0], right[0]) {
        (l, r) if l == r => check_order(&left[1..], &right[1..]),
        (_, ']') => Ordering::Less,
        (']', _) => Ordering::Greater,
        (_, '[') => {
            let converted_left = &*[&[left[0], ']'], &left[1..]].concat::<char>();
            return check_order(converted_left, &right[1..]);
        },
        ('[', _) => {
            let converted_right = &*[&[right[0], ']'], &right[1..]].concat::<char>();
            return check_order(&left[1..], converted_right);
        },
        (l, r) => if l <= r { Ordering::Greater } else { Ordering::Less },
    };
}

// enum Packet {
//     Int(u32),
//     List(Vec<Packet>),
// }
