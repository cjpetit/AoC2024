use std::{cmp::Ordering, env, fs::File, io::Read, path::Path};

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
    let (list1, list2) = partition(s);
    let sorted1 = mergesort(list1);
    let sorted2 = mergesort(list2);
    sum_of_diffs(sorted1, sorted2)
}

fn part2(s: &str) -> i32 {
    let (list1, list2) = partition(s);
    let mut similarity = 0;
    for x in list1.iter() {
        similarity += *x * appearances_in(*x, &list2);
    }
    similarity
}

fn partition(s: &str) -> (Vec<i32>, Vec<i32>) {
    let mut list1: Vec<i32> = Vec::new();
    let mut list2: Vec<i32> = Vec::new();
    let mut to1 = true;

    let mut start = 0;
    for (i, c) in s.chars().enumerate() {
        if !c.is_numeric() {
            if i - start >= 1 {
                let x: i32 = *&s[start..i].parse().expect("parse error");
                if to1 {
                    list1.push(x);
                    to1 = false;
                } else {
                    list2.push(x);
                    to1 = true;
                }
            }
            start = i+1;
        }
    }

    (list1, list2)
}

fn merge(l1: Vec<i32>, l2: Vec<i32>) -> Vec<i32> {
    let mut comb: Vec<i32> = Vec::new();
    let mut i1 = 0;
    let mut i2 = 0;
    loop {
        match (l1.get(i1), l2.get(i2)) {
            (None, None) => break,
            (Some(k1), Some(k2)) => {
                if k1 < k2 {
                    comb.push(*k1);
                    i1 += 1;
                } else {
                    comb.push(*k2);
                    i2 += 1;
                }
            },
            (Some(_), None) => {
                comb.extend(l1[i1..].into_iter());
                break;
            },
            (None, Some(_)) => {
                comb.extend(l2[i2..].into_iter());
                break;
            }
        }
    }
    comb
}

fn mergesort(l: Vec<i32>) -> Vec<i32> {
    let len = l.len();
    match len.cmp(&1) {
        Ordering::Less => vec![],
        Ordering::Equal => l,
        Ordering::Greater => {
            merge(mergesort(l[0..len/2].to_vec()), mergesort(l[len/2..len].to_vec()))
        }
    }
}

fn sum_of_diffs(list1: Vec<i32>, list2: Vec<i32>) -> i32 {
    let mut total = 0;
    for (i, x) in list1.iter().enumerate() {
        if let Some(y) = list2.get(i) {
            total += (y-x).abs();
            continue;
        }
        panic!("sum_of_diffs call on unequal length vectors");
    }
    total
}

fn appearances_in(x: i32, l: &Vec<i32>) -> i32 {
    let mut app = 0;
    for n in l.iter() {
        if *n == x {
            app += 1;
        }
    }
    app
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn merge_() {
        let l1 = vec![1, 4, 8, 12, 19];
        let l2 = vec![2, 3, 5, 17, 19, 22];
        assert_eq!(merge(l1, l2), vec![1, 2, 3, 4, 5, 8, 12, 17, 19, 19, 22])
    }

    #[test]
    fn mergesort_() {
        let l = vec![8, 3, 17, 5, 23, 18, 15, 17, 22, 3, 6];
        assert_eq!(mergesort(l), vec![3, 3, 5, 6, 8, 15, 17, 17, 18, 22, 23])
    }

    #[test]
    fn partition_() {
        let s = "1234 5678\n876 543\n22 333\n";
        assert_eq!(partition(s), (vec![1234, 876, 22], vec![5678, 543, 333]));
    }
}