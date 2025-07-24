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

fn part1(s: &str) -> u64 {
    let mut v = to_block_vec(s);
    move_blocks(&mut v);
    checksum(&v)
}

fn part2(s: &str) -> u64 {
    let mut v = to_block_vec(s);
    move_files(&mut v);
    checksum(&v)
}

fn to_block_vec(s: &str) -> Vec<Option<u32>> {
    let string = s.to_string();
    let bytes = string.trim().as_bytes();
    let mut v = Vec::new();
    let mut i = 0;
    let mut id = 0;
    loop {
        if i >= bytes.len() {break;}
        let c = bytes[i] as char;
        let n: u8 = String::from(c).parse().expect("Parse error");
        for _ in 0..n {
            v.push(Some(id))
        }
        if i + 1 >= bytes.len() {break;}
        let c = bytes[i+1] as char;
        let n: u8 = String::from(c).parse().expect("Parse error");
        for _ in 0..n {
            v.push(None)
        }
        i += 2;
        id += 1;
    }
    v
}

fn move_blocks(v: &mut Vec<Option<u32>>) {
    let mut empty_i = 0;
    loop {
        if empty_i >= v.len() {break;}
        if let Some(_) = v[empty_i] {empty_i += 1;}
        else {
            if let Some(Some(x)) = v.pop() {
                v[empty_i] = Some(x);
                empty_i += 1;
            }
        }
    }
}

fn move_files(v: &mut Vec<Option<u32>>) {
    let mut i = v.len() - 1;
    loop {
        if let Some(id) = v[i] {
            let mut width = 1;
            loop {
                if width > i {return;}
                if v[i-width] == Some(id) {width += 1;}
                else {break;}
            };
            i -= width;
            if let Some(i_to) = find_vacancy(v, width, i+1) {
                transpose_blocks(v, i+1, i_to, width);
            }
        } else {
            if i == 0 {return;}
            i -= 1;
        }
    }
}

fn find_vacancy(v: &Vec<Option<u32>>, width: usize, left_of: usize) -> Option<usize> {
    let mut empties = 0;
    for (i, elem) in v[0..left_of].iter().enumerate() {
        if let None = elem {empties += 1;}
        else {empties = 0;}
        if empties == width {return Some(i+1-width);}
    }
    None
}

fn checksum(v: &Vec<Option<u32>>) -> u64 {
    let mut sum = 0;
    for (i, opt) in v.iter().enumerate() {
        if let Some(x) = *opt {
            sum += (i as u32 * x) as u64;
        }
    }
    sum
}

fn transpose_blocks(v: &mut Vec<Option<u32>>, i_from: usize, i_to: usize, n: usize) {
    let mut n = n;
    let mut i_from = i_from;
    let mut i_to = i_to;
    while n > 0 {
        if let Some(_) = v[i_to] {panic!("Errant transpose");}
        v[i_to] = v[i_from];
        v[i_from] = None;
        i_to += 1;
        i_from += 1;
        n -= 1;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn to_block_vec_() {
        let s = "123110312\n";
        let expected = vec![Some(0), None, None, Some(1), Some(1), Some(1), None, Some(2), Some(3), Some(3), Some(3), None, Some(4), Some(4)];
        assert_eq!(to_block_vec(s), expected);
    }

    #[test]
    fn move_blocks_() {
        let mut v = vec![Some(0), None, None, Some(1), Some(1), Some(1), None, Some(2), Some(3), Some(3), Some(3), None, Some(4), Some(4)];
        move_blocks(&mut v);
        let expected = vec![Some(0), Some(4), Some(4), Some(1), Some(1), Some(1), Some(3), Some(2), Some(3), Some(3)];
        assert_eq!(v, expected);
    }

    #[test]
    fn checksum_() {
        let v = vec![Some(0), Some(4), Some(4), Some(1), Some(1), Some(1), Some(3), Some(2), Some(3), Some(3)];
        assert_eq!(checksum(&v), 107);
    }

    #[test]
    fn transpose_blocks_() {
        let mut v = vec![Some(0), None, None, Some(1), Some(1), Some(1), None, Some(2), Some(2), Some(3), Some(3), None, Some(4), Some(4)];
        let expected = vec![Some(0), Some(2), Some(2), Some(1), Some(1), Some(1), None, None, None, Some(3), Some(3), None, Some(4), Some(4)];
        transpose_blocks(&mut v, 7, 1, 2);
        assert_eq!(v, expected);
    }

    #[test]
    fn find_vacancy_() {
        let v = vec![Some(0), None, None, Some(1), Some(1), Some(1), None, Some(2), Some(2), Some(3), Some(3), None, None, None, Some(4), Some(4)];
        assert_eq!(find_vacancy(&v, 2, 9), Some(1));
        assert_eq!(find_vacancy(&v, 3, 9), None);
    }

    #[test]
    fn move_files_() {
        let mut v = vec![Some(0), Some(0), None, None, None, Some(1), Some(1), Some(1), None, None, None, Some(2), None,
            None, None, Some(3), Some(3), Some(3), None, Some(4), Some(4), None, Some(5), Some(5), Some(5), Some(5), None, Some(6), Some(6),
            Some(6), Some(6), None, Some(7), Some(7), Some(7), None, Some(8), Some(8), Some(8), Some(8), Some(9), Some(9)];
        let expected = vec![Some(0), Some(0), Some(9), Some(9), Some(2), Some(1), Some(1), Some(1), Some(7), Some(7), Some(7),
            None, Some(4), Some(4), None, Some(3), Some(3), Some(3), None, None, None, None, Some(5), Some(5), Some(5), Some(5), None,
            Some(6), Some(6), Some(6), Some(6), None, None, None, None, None, Some(8), Some(8), Some(8), Some(8), None, None];
        move_files(&mut v);
        assert_eq!(v, expected);
        assert_eq!(checksum(&v), 2858);
    }
}