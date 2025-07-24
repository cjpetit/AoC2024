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

fn part1(s: &str) -> u32 {
    let mut score = 0;
    let map = scan_to_map(s);
    for (coord, dig) in map.iter() {
        if *dig == 0 {
            let ones = find_ones(&map, *coord);
            let twos = find_twos(&map, ones);
            let threes = find_threes(&map, twos);
            let fours = find_fours(&map, threes);
            let fives = find_fives(&map, fours);
            let sixs = find_sixs(&map, fives);
            let sevens = find_sevens(&map, sixs);
            let eights = find_eights(&map, sevens);
            let nines = find_nines(&map, eights);
            score += nines.len() as u32;
        }
    }
    score
}

fn part2(s: &str) -> u32 {
    let mut ratings = 0;
    let map = scan_to_map(s);
    for (coord, dig) in map.iter() {
        if *dig == 0 {
            ratings += rating(&map, *coord);
        }
    }
    ratings
}

fn scan_to_map(s: &str) -> HashMap<(u32, u32), u8> {
    let mut map: HashMap<(u32, u32), u8> = HashMap::new();
    let mut i = 0;
    let mut j = 0;
    for c in s.chars() {
        match c {
            '\n' => {
                i += 1;
                j = 0;
            },
            '\r' => {},
            dig => {
                let n: u8 = String::from(dig).parse().expect("parse error");
                map.insert((i, j), n);
                j += 1;
            }
        }
    }
    map
}

fn find_ones(map: &HashMap<(u32, u32), u8>, zero: (u32, u32)) -> Vec<(u32, u32)> {
    let mut ones: Vec<(u32, u32)> = Vec::new();
    if zero.0 > 0 {
        let up = (zero.0 - 1, zero.1);
        if map.get(&up) == Some(&1) {ones.push(up);}
    }
    if zero.1 > 0 {
        let left = (zero.0, zero.1 - 1);
        if map.get(&left) == Some(&1) {ones.push(left);}
    }
    let down = (zero.0 + 1, zero.1);
    let right = (zero.0, zero.1 + 1);
    if map.get(&down) == Some(&1) {ones.push(down);}
    if map.get(&right) == Some(&1) {ones.push(right);}
    ones
}

fn find_twos(map: &HashMap<(u32, u32), u8>, ones: Vec<(u32, u32)>) -> Vec<(u32, u32)> {
    let mut twos: Vec<(u32, u32)> = Vec::new();
    for one in ones.iter() {
        if one.0 > 0 {
            let up = (one.0 - 1, one.1);
            if map.get(&up) == Some(&2) && !twos.contains(&up) {twos.push(up);}
        }
        if one.1 > 0 {
            let left = (one.0, one.1 - 1);
            if map.get(&left) == Some(&2) && !twos.contains(&left) {twos.push(left);}
        }
        let down = (one.0 + 1, one.1);
        let right = (one.0, one.1 + 1);
        if map.get(&down) == Some(&2) && !twos.contains(&down) {twos.push(down);}
        if map.get(&right) == Some(&2) && !twos.contains(&right) {twos.push(right);}
    }
    twos
}

fn find_threes(map: &HashMap<(u32, u32), u8>, twos: Vec<(u32, u32)>) -> Vec<(u32, u32)> {
    let mut threes: Vec<(u32, u32)> = Vec::new();
    for two in twos.iter() {
        if two.0 > 0 {
            let up = (two.0 - 1, two.1);
            if map.get(&up) == Some(&3) && !threes.contains(&up) {threes.push(up);}
        }
        if two.1 > 0 {
            let left = (two.0, two.1 - 1);
            if map.get(&left) == Some(&3) && !threes.contains(&left) {threes.push(left);}
        }
        let down = (two.0 + 1, two.1);
        let right = (two.0, two.1 + 1);
        if map.get(&down) == Some(&3) && !threes.contains(&down) {threes.push(down);}
        if map.get(&right) == Some(&3) && !threes.contains(&right) {threes.push(right);}
    }
    threes
}

fn find_fours(map: &HashMap<(u32, u32), u8>, threes: Vec<(u32, u32)>) -> Vec<(u32, u32)> {
    let mut fours: Vec<(u32, u32)> = Vec::new();
    for three in threes.iter() {
        if three.0 > 0 {
            let up = (three.0 - 1, three.1);
            if map.get(&up) == Some(&4) && !fours.contains(&up) {fours.push(up);}
        }
        if three.1 > 0 {
            let left = (three.0, three.1 - 1);
            if map.get(&left) == Some(&4) && !fours.contains(&left) {fours.push(left);}
        }
        let down = (three.0 + 1, three.1);
        let right = (three.0, three.1 + 1);
        if map.get(&down) == Some(&4) && !fours.contains(&down) {fours.push(down);}
        if map.get(&right) == Some(&4) && !fours.contains(&right) {fours.push(right);}
    }
    fours
}

fn find_fives(map: &HashMap<(u32, u32), u8>, fours: Vec<(u32, u32)>) -> Vec<(u32, u32)> {
    let mut fives: Vec<(u32, u32)> = Vec::new();
    for four in fours.iter() {
        if four.0 > 0 {
            let up = (four.0 - 1, four.1);
            if map.get(&up) == Some(&5) && !fives.contains(&up) {fives.push(up);}
        }
        if four.1 > 0 {
            let left = (four.0, four.1 - 1);
            if map.get(&left) == Some(&5) && !fives.contains(&left) {fives.push(left);}
        }
        let down = (four.0 + 1, four.1);
        let right = (four.0, four.1 + 1);
        if map.get(&down) == Some(&5) && !fives.contains(&down) {fives.push(down);}
        if map.get(&right) == Some(&5) && !fives.contains(&right) {fives.push(right);}
    }
    fives
}

fn find_sixs(map: &HashMap<(u32, u32), u8>, fives: Vec<(u32, u32)>) -> Vec<(u32, u32)> {
    let mut sixs: Vec<(u32, u32)> = Vec::new();
    for five in fives.iter() {
        if five.0 > 0 {
            let up = (five.0 - 1, five.1);
            if map.get(&up) == Some(&6) && !sixs.contains(&up) {sixs.push(up);}
        }
        if five.1 > 0 {
            let left = (five.0, five.1 - 1);
            if map.get(&left) == Some(&6) && !sixs.contains(&left) {sixs.push(left);}
        }
        let down = (five.0 + 1, five.1);
        let right = (five.0, five.1 + 1);
        if map.get(&down) == Some(&6) && !sixs.contains(&down) {sixs.push(down);}
        if map.get(&right) == Some(&6) && !sixs.contains(&right) {sixs.push(right);}
    }
    sixs
}

fn find_sevens(map: &HashMap<(u32, u32), u8>, sixs: Vec<(u32, u32)>) -> Vec<(u32, u32)> {
    let mut sevens: Vec<(u32, u32)> = Vec::new();
    for six in sixs.iter() {
        if six.0 > 0 {
            let up = (six.0 - 1, six.1);
            if map.get(&up) == Some(&7) && !sevens.contains(&up) {sevens.push(up);}
        }
        if six.1 > 0 {
            let left = (six.0, six.1 - 1);
            if map.get(&left) == Some(&7) && !sevens.contains(&left) {sevens.push(left);}
        }
        let down = (six.0 + 1, six.1);
        let right = (six.0, six.1 + 1);
        if map.get(&down) == Some(&7) && !sevens.contains(&down) {sevens.push(down);}
        if map.get(&right) == Some(&7) && !sevens.contains(&right) {sevens.push(right);}
    }
    sevens
}

fn find_eights(map: &HashMap<(u32, u32), u8>, sevens: Vec<(u32, u32)>) -> Vec<(u32, u32)> {
    let mut eights: Vec<(u32, u32)> = Vec::new();
    for seven in sevens.iter() {
        if seven.0 > 0 {
            let up = (seven.0 - 1, seven.1);
            if map.get(&up) == Some(&8) && !eights.contains(&up) {eights.push(up);}
        }
        if seven.1 > 0 {
            let left = (seven.0, seven.1 - 1);
            if map.get(&left) == Some(&8) && !eights.contains(&left) {eights.push(left);}
        }
        let down = (seven.0 + 1, seven.1);
        let right = (seven.0, seven.1 + 1);
        if map.get(&down) == Some(&8) && !eights.contains(&down) {eights.push(down);}
        if map.get(&right) == Some(&8) && !eights.contains(&right) {eights.push(right);}
    }
    eights
}

fn find_nines(map: &HashMap<(u32, u32), u8>, eights: Vec<(u32, u32)>) -> Vec<(u32, u32)> {
    let mut nines: Vec<(u32, u32)> = Vec::new();
    for eight in eights.iter() {
        if eight.0 > 0 {
            let up = (eight.0 - 1, eight.1);
            if map.get(&up) == Some(&9) && !nines.contains(&up) {nines.push(up);}
        }
        if eight.1 > 0 {
            let left = (eight.0, eight.1 - 1);
            if map.get(&left) == Some(&9) && !nines.contains(&left) {nines.push(left);}
        }
        let down = (eight.0 + 1, eight.1);
        let right = (eight.0, eight.1 + 1);
        if map.get(&down) == Some(&9) && !nines.contains(&down) {nines.push(down);}
        if map.get(&right) == Some(&9) && !nines.contains(&right) {nines.push(right);}
    }
    nines
}

fn rating(map: &HashMap<(u32, u32), u8>, zero: (u32, u32)) -> u32 {
    let mut rating = 0;
    for one in find_ones(map, zero).iter() {
        for two in find_twos_single(map, *one).iter() {
            for three in find_threes_single(map, *two).iter() {
                for four in find_fours_single(map, *three).iter() {
                    for five in find_fives_single(map, *four).iter() {
                        for six in find_sixs_single(map, *five).iter() {
                            for seven in find_sevens_single(map, *six).iter() {
                                for eight in find_eights_single(map, *seven).iter() {
                                    for _ in find_nines_single(map, *eight).iter() {
                                        rating += 1;
                                    }                        
                                }                    
                            }                
                        }            
                    }        
                }    
            }
        }
    }
    rating
}

fn find_twos_single(map: &HashMap<(u32, u32), u8>, one: (u32, u32)) -> Vec<(u32, u32)> {
    let mut twos: Vec<(u32, u32)> = Vec::new();
    if one.0 > 0 {
        let up = (one.0 - 1, one.1);
        if map.get(&up) == Some(&2) && !twos.contains(&up) {twos.push(up);}
    }
    if one.1 > 0 {
        let left = (one.0, one.1 - 1);
        if map.get(&left) == Some(&2) && !twos.contains(&left) {twos.push(left);}
    }
    let down = (one.0 + 1, one.1);
    let right = (one.0, one.1 + 1);
    if map.get(&down) == Some(&2) && !twos.contains(&down) {twos.push(down);}
    if map.get(&right) == Some(&2) && !twos.contains(&right) {twos.push(right);}
    twos
}

fn find_threes_single(map: &HashMap<(u32, u32), u8>, two: (u32, u32)) -> Vec<(u32, u32)> {
    let mut threes: Vec<(u32, u32)> = Vec::new();
    if two.0 > 0 {
        let up = (two.0 - 1, two.1);
        if map.get(&up) == Some(&3) && !threes.contains(&up) {threes.push(up);}
    }
    if two.1 > 0 {
        let left = (two.0, two.1 - 1);
        if map.get(&left) == Some(&3) && !threes.contains(&left) {threes.push(left);}
    }
    let down = (two.0 + 1, two.1);
    let right = (two.0, two.1 + 1);
    if map.get(&down) == Some(&3) && !threes.contains(&down) {threes.push(down);}
    if map.get(&right) == Some(&3) && !threes.contains(&right) {threes.push(right);}
    threes
}

fn find_fours_single(map: &HashMap<(u32, u32), u8>, three: (u32, u32)) -> Vec<(u32, u32)> {
    let mut fours: Vec<(u32, u32)> = Vec::new();
    if three.0 > 0 {
        let up = (three.0 - 1, three.1);
        if map.get(&up) == Some(&4) && !fours.contains(&up) {fours.push(up);}
    }
    if three.1 > 0 {
        let left = (three.0, three.1 - 1);
        if map.get(&left) == Some(&4) && !fours.contains(&left) {fours.push(left);}
    }
    let down = (three.0 + 1, three.1);
    let right = (three.0, three.1 + 1);
    if map.get(&down) == Some(&4) && !fours.contains(&down) {fours.push(down);}
    if map.get(&right) == Some(&4) && !fours.contains(&right) {fours.push(right);}
    fours
}

fn find_fives_single(map: &HashMap<(u32, u32), u8>, four: (u32, u32)) -> Vec<(u32, u32)> {
    let mut fives: Vec<(u32, u32)> = Vec::new();
    if four.0 > 0 {
        let up = (four.0 - 1, four.1);
        if map.get(&up) == Some(&5) && !fives.contains(&up) {fives.push(up);}
    }
    if four.1 > 0 {
        let left = (four.0, four.1 - 1);
        if map.get(&left) == Some(&5) && !fives.contains(&left) {fives.push(left);}
    }
    let down = (four.0 + 1, four.1);
    let right = (four.0, four.1 + 1);
    if map.get(&down) == Some(&5) && !fives.contains(&down) {fives.push(down);}
    if map.get(&right) == Some(&5) && !fives.contains(&right) {fives.push(right);}
    fives
}

fn find_sixs_single(map: &HashMap<(u32, u32), u8>, five: (u32, u32)) -> Vec<(u32, u32)> {
    let mut sixs: Vec<(u32, u32)> = Vec::new();
    if five.0 > 0 {
        let up = (five.0 - 1, five.1);
        if map.get(&up) == Some(&6) && !sixs.contains(&up) {sixs.push(up);}
    }
    if five.1 > 0 {
        let left = (five.0, five.1 - 1);
        if map.get(&left) == Some(&6) && !sixs.contains(&left) {sixs.push(left);}
    }
    let down = (five.0 + 1, five.1);
    let right = (five.0, five.1 + 1);
    if map.get(&down) == Some(&6) && !sixs.contains(&down) {sixs.push(down);}
    if map.get(&right) == Some(&6) && !sixs.contains(&right) {sixs.push(right);}
    sixs
}

fn find_sevens_single(map: &HashMap<(u32, u32), u8>, six: (u32, u32)) -> Vec<(u32, u32)> {
    let mut sevens: Vec<(u32, u32)> = Vec::new();
    if six.0 > 0 {
        let up = (six.0 - 1, six.1);
        if map.get(&up) == Some(&7) && !sevens.contains(&up) {sevens.push(up);}
    }
    if six.1 > 0 {
        let left = (six.0, six.1 - 1);
        if map.get(&left) == Some(&7) && !sevens.contains(&left) {sevens.push(left);}
    }
    let down = (six.0 + 1, six.1);
    let right = (six.0, six.1 + 1);
    if map.get(&down) == Some(&7) && !sevens.contains(&down) {sevens.push(down);}
    if map.get(&right) == Some(&7) && !sevens.contains(&right) {sevens.push(right);}
    sevens
}

fn find_eights_single(map: &HashMap<(u32, u32), u8>, seven: (u32, u32)) -> Vec<(u32, u32)> {
    let mut eights: Vec<(u32, u32)> = Vec::new();
    if seven.0 > 0 {
        let up = (seven.0 - 1, seven.1);
        if map.get(&up) == Some(&8) && !eights.contains(&up) {eights.push(up);}
    }
    if seven.1 > 0 {
        let left = (seven.0, seven.1 - 1);
        if map.get(&left) == Some(&8) && !eights.contains(&left) {eights.push(left);}
    }
    let down = (seven.0 + 1, seven.1);
    let right = (seven.0, seven.1 + 1);
    if map.get(&down) == Some(&8) && !eights.contains(&down) {eights.push(down);}
    if map.get(&right) == Some(&8) && !eights.contains(&right) {eights.push(right);}
    eights
}

fn find_nines_single(map: &HashMap<(u32, u32), u8>, eight: (u32, u32)) -> Vec<(u32, u32)> {
    let mut nines: Vec<(u32, u32)> = Vec::new();
    if eight.0 > 0 {
        let up = (eight.0 - 1, eight.1);
        if map.get(&up) == Some(&9) && !nines.contains(&up) {nines.push(up);}
    }
    if eight.1 > 0 {
        let left = (eight.0, eight.1 - 1);
        if map.get(&left) == Some(&9) && !nines.contains(&left) {nines.push(left);}
    }
    let down = (eight.0 + 1, eight.1);
    let right = (eight.0, eight.1 + 1);
    if map.get(&down) == Some(&9) && !nines.contains(&down) {nines.push(down);}
    if map.get(&right) == Some(&9) && !nines.contains(&right) {nines.push(right);}
    nines
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn scan_to_map_() {
        let map = scan_to_map("0123\n1234\n8765\n9876\n");
        assert_eq!(map.get(&(0, 0)), Some(&0));
        assert_eq!(map.get(&(2, 1)), Some(&7));
        assert_eq!(map.get(&(4, 2)), None);
    }

    #[test]
    fn find_ones_() {
        let map = scan_to_map("0123\n1234\n8765\n9876\n");
        let ones = find_ones(&map, (0, 0));
        assert_eq!(ones.len(), 2);
        assert!(ones.contains(&(0, 1)));
        assert!(ones.contains(&(1, 0)));
    }

    #[test]
    fn find_twos_() {
        let map = scan_to_map("0123\n1234\n8765\n9876\n");
        let ones = vec![(0, 1), (1, 0)];
        let twos = find_twos(&map, ones);
        assert_eq!(twos.len(), 2);
        assert!(twos.contains(&(1, 1)));
        assert!(twos.contains(&(0, 2)));
    }
}