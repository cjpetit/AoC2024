use std::{env, fs::File, io::Read, path::Path, collections::HashMap};

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut input = String::new();
    match args.len() {
        2 => {
            let path = Path::new(args[1].as_str());
            let mut file = File::open(&path).expect(format!("Error: could not open file '{}'", args[1].as_str()).as_str());
            file.read_to_string(&mut input).expect("could not read input");
        },
        _ => panic!("Usage: cargo run -- filename.txt\n")
    };
    
    println!("Part 1: {}", part1(input.as_str()));
    println!("Part 2: {}", part2(input.as_str()));
}

fn part1(s: &str) -> u64 {
    let mut stones = read_to_hashmap(s);
    for _ in 0..25 {
        stones = blink(&stones);
        // println!("{:?}", stones);
    }
    num_stones(&stones)
}

fn part2(s: &str) -> u64 {
    let mut stones = read_to_hashmap(s);
    for _ in 0..75 {
        stones = blink(&stones);
        // println!("{:?}", stones);
    }
    num_stones(&stones)
}

// fn read_to_vec(s: &str) -> Vec<u64> {
//     let mut stones = vec![];
//     for stone in s.trim().split(" ") {
//         stones.push(stone.parse::<u64>().expect("parse error"));
//     }
//     stones
// }

fn read_to_hashmap(s: &str) -> HashMap<u64, u64> {
    let mut stones = HashMap::new();
    for stone in s.trim().split(" ") {
        let val: u64 = stone.parse().expect("parse error");
        add_stone(&mut stones, (val, 1));
    }
    stones
}

fn digits(x: u64) -> u8 {
    if x == 0 {0}
    else {1 + digits(x/10)}
}

fn split(x: u64, n: u8) -> (u64, u64) {
    (x / u64::pow(10, n as u32), x % u64::pow(10, n as u32))
}

fn blink(v: &HashMap<u64, u64>) -> HashMap<u64, u64> {
    let mut new_map: HashMap<u64, u64> = HashMap::new();
    for (stone, freq) in v.iter() {
        if *stone == 0 {
            add_stone(&mut new_map, (1, *freq));
        } else {
            let digs = digits(*stone);
            if digs % 2 == 0 {
                let (left, right) = split(*stone, digs / 2);
                add_stone(&mut new_map, (left, *freq));
                add_stone(&mut new_map, (right, *freq));
            } else {
                add_stone(&mut new_map, (*stone * 2024, *freq));
            }
        }
    }
    new_map
}

fn add_stone(map: &mut HashMap<u64, u64>, stone: (u64, u64)) {
    if let Some(s_ref) = map.get_mut(&stone.0) {
        *s_ref += stone.1;
    } else {
        map.insert(stone.0, stone.1);
    }
}

fn num_stones(stones: &HashMap<u64, u64>) -> u64 {
    let mut num_stones = 0;
    for (_, freq) in stones.iter() {
        num_stones += *freq;
    }
    num_stones
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn read_to_hashmap_() {
        let v = read_to_hashmap("253 0 2024 14168");
        let expected = HashMap::from([(253, 1), (0, 1), (2024, 1), (14168, 1)]);
        assert_eq!(v, expected);
    }

    #[test]
    fn blink_() {
        let mut v = HashMap::from([(253, 1), (0, 1), (2024, 1), (14168, 1)]);
        let expected = HashMap::from([(512072, 1), (1, 1), (20, 1), (24, 1), (28676032, 1)]);
        v = blink(&v);
        assert_eq!(v, expected);
    }
}