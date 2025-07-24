use std::{env, fs::File, io::Read, path::Path};

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
    let mut num_safe = 0;
    let mut s = s;
    loop {
        let (report, i) = read_report(s, 0);
        num_safe += is_safe(&report) as i32;
        if i == 0 {break;}
        s = &s[i..];
    }
    num_safe
}

fn part2(s: &str) -> i32 {
    let mut num_safe = 0;
    let mut s = s;
    loop {
        let (report, i) = read_report(s, 0);
        if is_safe(&report) {
            num_safe += 1;
        } else {
            for removal in every_removal(&report).iter() {
                if is_safe(removal) {
                    num_safe += 1;
                    break;
                }
            }
        }
        if i == 0 {break;}
        s = &s[i..];
    }
    num_safe
}

fn read_report(s: &str, start: usize) -> (Vec<i32>, usize) {
    let mut start = start;
    let mut levels: Vec<i32> = Vec::new();
    for (i, c) in s.chars().enumerate() {
        if !c.is_numeric() {
            if i - start > 1 || start == 0 {
                let trimmed = *&s[start..i].trim();
                if trimmed != "" {
                    let x: i32 = trimmed.parse().expect(format!("parse error: '{}'", trimmed.to_string()).as_str());
                    levels.push(x);
                }
            }
            if c == '\n' {return (levels, i+1);}
            start = i;
        } else if i == s.len()-1 {
            let trimmed = *&s[start..=i].trim();
            if trimmed != "" {
                let x: i32 = trimmed.parse().expect(format!("parse error: '{}'", trimmed.to_string()).as_str());
                levels.push(x);
            }
        }
    }
    (levels, 0)
}

fn is_safe(l: &Vec<i32>) -> bool {
    let mut greater = false;
    let mut less = false;
    for i in 1..l.len() {
        if l[i] > l[i-1] && l[i] - l[i-1] <= 3 && !less {
            greater = true;
        } else if l[i] < l[i-1] && l[i-1] - l[i] <= 3 && !greater {
            less = true;
        } else {return false;}
    }
    true
}

fn every_removal(l: &Vec<i32>) -> Vec<Vec<i32>> {
    let len_l = l.len();
    if len_l < 2 {panic!("report with less than 2 levels");}
    let mut removals: Vec<Vec<i32>> = Vec::new();
    removals.push(l[1..].to_vec());
    for i in 1..len_l-1 {
        removals.push([&l[..i], &l[i+1..]].concat());
    }
    removals.push(l[..len_l-1].to_vec());
    removals
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn read_report_() {
        let s = " 7 6 4 2 1\n1 2 7 8 9\n9 7 6 2 1";
        assert_eq!(read_report(s, 0).0, vec![7, 6, 4, 2, 1]);
    }

    #[test]
    fn is_safe_1() {
        let s = " 7 6 4 2 1";
        assert!(is_safe(&read_report(s, 0).0));
    }

    #[test]
    fn is_safe_2() {
        let s = "1 2 7 8 9";
        assert!(!is_safe(&read_report(s, 0).0));
    }

    #[test]
    fn is_safe_3() {
        let s = "9 7 6 2 1";
        assert!(!is_safe(&read_report(s, 0).0));
    }

    #[test]
    fn is_safe_4() {
        let s = "1 3 2 4 5";
        assert!(!is_safe(&read_report(s, 0).0));
    }

    #[test]
    fn is_safe_5() {
        let s = "8 6 4 4 1";
        assert!(!is_safe(&read_report(s, 0).0));
    }

    #[test]
    fn is_safe_6() {
        let s = "1 3 6 7 9";
        assert!(is_safe(&read_report(s, 0).0));
    }

    #[test]
    fn every_removal_() {
        let v = vec![1, 2, 3, 4];
        assert_eq!(every_removal(&v), vec![vec![2, 3, 4], vec![1, 3, 4], vec![1, 2, 4], vec![1, 2, 3]]);
    }
}