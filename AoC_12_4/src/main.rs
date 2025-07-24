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

fn part1(s: &str) -> i32 {
    let mut num = 0;
    let (map, max_i, max_j) = scan_to_map(s);
    for i in 0..=max_i {
        for j in 0..=max_j {
            num += scan_everywhere(&map, i, j);
        }
    }
    num
}

fn part2(s: &str) -> i32 {
    let mut num = 0;
    let (map, max_i, max_j) = scan_to_map(s);
    for i in 1..max_i {
        for j in 1..max_j {
            if let Some(&'A') = map.get(&(i, j)) {num += check_a(&map, i, j) as i32;}
        }
    }
    num
}

fn scan_to_map(s: &str) -> (HashMap<(i32, i32), char>, i32, i32) {
    let mut map: HashMap<(i32, i32), char> = HashMap::new();
    let mut i = 0;
    let mut j = 0;
    let mut max_j = 0;
    for c in s.chars() {
        match c {
            'X' => {
                map.insert((i, j), 'X');
                j += 1;
            },
            'M' => {
                map.insert((i, j), 'M');
                j += 1;
            },
            'A' => {
                map.insert((i, j), 'A');
                j += 1;
            },
            'S' => {
                map.insert((i, j), 'S');
                j += 1;
            },
            '\n' => {
                i += 1;
                if j-1 > max_j {max_j = j-1;}
                j = 0;
            },
            _ => {j += 1;}
        }
    }
    (map, i-1, max_j)
}

fn scan_everywhere(map: &HashMap<(i32, i32), char>, i: i32, j: i32) -> i32 {
    let mut found = scan_right(map, i, j) as i32;
    found += scan_down(map, i, j) as i32;
    found += scan_left(map, i, j) as i32;
    found += scan_up(map, i, j) as i32;
    found += scan_up_left(map, i, j) as i32;
    found += scan_up_right(map, i, j) as i32;
    found += scan_down_left(map, i, j) as i32;
    found += scan_down_right(map, i, j) as i32;
    found
}

fn scan_right(map: &HashMap<(i32, i32), char>, i: i32, j: i32) -> bool {
    if let Some(&'X') = map.get(&(i, j)) {
        if let Some(&'M') = map.get(&(i, j+1)) {
            if let Some(&'A') = map.get(&(i, j+2)) {
                if let Some(&'S') = map.get(&(i, j+3)) {
                    return true;
                }        
            }    
        }
    }
    false
}

fn scan_down(map: &HashMap<(i32, i32), char>, i: i32, j: i32) -> bool {
    if let Some(&'X') = map.get(&(i, j)) {
        if let Some(&'M') = map.get(&(i+1, j)) {
            if let Some(&'A') = map.get(&(i+2, j)) {
                if let Some(&'S') = map.get(&(i+3, j)) {
                    return true;
                }        
            }    
        }
    }
    false
}

fn scan_left(map: &HashMap<(i32, i32), char>, i: i32, j: i32) -> bool {
    if let Some(&'X') = map.get(&(i, j)) {
        if let Some(&'M') = map.get(&(i, j-1)) {
            if let Some(&'A') = map.get(&(i, j-2)) {
                if let Some(&'S') = map.get(&(i, j-3)) {
                    return true;
                }        
            }    
        }
    }
    false
}

fn scan_up(map: &HashMap<(i32, i32), char>, i: i32, j: i32) -> bool {
    if let Some(&'X') = map.get(&(i, j)) {
        if let Some(&'M') = map.get(&(i-1, j)) {
            if let Some(&'A') = map.get(&(i-2, j)) {
                if let Some(&'S') = map.get(&(i-3, j)) {
                    return true;
                }        
            }    
        }
    }
    false
}

fn scan_up_left(map: &HashMap<(i32, i32), char>, i: i32, j: i32) -> bool {
    if let Some(&'X') = map.get(&(i, j)) {
        if let Some(&'M') = map.get(&(i-1, j-1)) {
            if let Some(&'A') = map.get(&(i-2, j-2)) {
                if let Some(&'S') = map.get(&(i-3, j-3)) {
                    return true;
                }        
            }    
        }
    }
    false
}

fn scan_up_right(map: &HashMap<(i32, i32), char>, i: i32, j: i32) -> bool {
    if let Some(&'X') = map.get(&(i, j)) {
        if let Some(&'M') = map.get(&(i-1, j+1)) {
            if let Some(&'A') = map.get(&(i-2, j+2)) {
                if let Some(&'S') = map.get(&(i-3, j+3)) {
                    return true;
                }        
            }    
        }
    }
    false
}

fn scan_down_left(map: &HashMap<(i32, i32), char>, i: i32, j: i32) -> bool {
    if let Some(&'X') = map.get(&(i, j)) {
        if let Some(&'M') = map.get(&(i+1, j-1)) {
            if let Some(&'A') = map.get(&(i+2, j-2)) {
                if let Some(&'S') = map.get(&(i+3, j-3)) {
                    return true;
                }        
            }    
        }
    }
    false
}

fn scan_down_right(map: &HashMap<(i32, i32), char>, i: i32, j: i32) -> bool {
    if let Some(&'X') = map.get(&(i, j)) {
        if let Some(&'M') = map.get(&(i+1, j+1)) {
            if let Some(&'A') = map.get(&(i+2, j+2)) {
                if let Some(&'S') = map.get(&(i+3, j+3)) {
                    return true;
                }        
            }    
        }
    }
    false
}

fn check_a(map: &HashMap<(i32, i32), char>, i: i32, j: i32) -> bool {
    if let (Some(ul), Some(ur), Some(dr), Some(dl)) = (map.get(&(i-1, j-1)), map.get(&(i-1, j+1)), map.get(&(i+1, j+1)), map.get(&(i+1, j-1))) {
        ((*ul == 'M' && *dr == 'S') || (*ul == 'S' && *dr == 'M')) && ((*ur == 'M' && *dl == 'S') || (*ur == 'S' && *dl == 'M'))
    } else {false}
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn scan_to_map_() {
        let s = "MMM\nXMS\nAXM\n";
        assert_eq!(scan_to_map(s), (HashMap::from([((0, 0), 'M'), ((0, 1), 'M'), ((0, 2), 'M'), ((1, 0), 'X'), ((1, 1), 'M'), ((1, 2), 'S'), ((2, 0), 'A'), ((2, 1), 'X'), ((2, 2), 'M'), ]), 2, 2));
    }

    #[test]
    fn part1_() {
        let s = "MMMSXXMASM\nMSAMXMSMSA\nAMXSXMAAMM\nMSAMASMSMX\nXMASAMXAMM\nXXAMMXXAMA\nSMSMSASXSS\nSAXAMASAAA\nMAMMMXMMMM\nMXMXAXMASX\n";
        assert_eq!(part1(s), 18);
    }

    #[test]
    fn check_a_() {
        let s = "MMMSXXMASM\nMSAMXMSMSA\nAMXSXMAAMM\nMSAMASMSMX\nXMASAMXAMM\nXXAMMXXAMA\nSMSMSASXSS\nSAXAMASAAA\nMAMMMXMMMM\nMXMXAXMASX\n";
        let (map, _, _) = scan_to_map(s);
        assert!(check_a(&map, 1, 2));
        assert!(!check_a(&map, 1, 9));
    }

    #[test]
    fn part2_() {
        let s = "MMMSXXMASM\nMSAMXMSMSA\nAMXSXMAAMM\nMSAMASMSMX\nXMASAMXAMM\nXXAMMXXAMA\nSMSMSASXSS\nSAXAMASAAA\nMAMMMXMMMM\nMXMXAXMASX\n";
        assert_eq!(part2(s), 9);
    }
}