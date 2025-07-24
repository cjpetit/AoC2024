use std::{env, fs::File, io::Read, path::Path};

#[derive(Clone)]
struct Map {
    width: i32,
    height: i32,
    signals: Vec<Signal>,
    antinodes: Vec<(i32, i32)>
}

impl Map {
    
    fn from_str(s: &str) -> Self {
        let mut width = 0;
        let mut height = 0;
        let mut signals = Vec::new();
        let mut x = 0;
        for byte in s.as_bytes() {
            match *byte as char {
                '.' => {x += 1;},
                '\n' => {
                    if width == 0 {width = x;}
                    else if width != x {panic!("Inconsistent width while scanning map")}
                    height += 1;
                    x = 0;
                },
                '\r' => {},
                c => {
                    let signal = Signal {
                        loc: (height, x),
                        c
                    };
                    signals.push(signal);
                    x += 1;
                }
            }
        }

        Self {
            width,
            height,
            signals,
            antinodes: Vec::new()
        }
    }

    fn pairs(&self, c: char) -> Vec<(Signal, Signal)> {
        let mut c_signals: Vec<Signal> = Vec::new();
        let mut pairs: Vec<(Signal, Signal)> = Vec::new();
        for signal in self.signals.iter() {
            if signal.c == c {
                c_signals.push(*signal);
            }
        }
        while c_signals.len() > 1 {
            let s = c_signals.pop().unwrap();
            for other in c_signals.iter() {
                pairs.push((s, *other));
            }
        }
        pairs
    }

    fn chars_used(&self) -> Vec<char> {
        let mut chars: Vec<char> = Vec::new();
        for signal in self.signals.iter() {
            let c = signal.c;
            if !chars.contains(&c) {chars.push(c)}
        }
        chars
    }

    fn place_antinodes_1(&mut self) {
        let chars = self.chars_used();
        for c in chars.iter() {
            let c_pairs = self.pairs(*c);
            for pair in c_pairs.iter() {
                let dx = pair.1.loc.0 - pair.0.loc.0;
                let dy = pair.1.loc.1 - pair.0.loc.1;
                let an1 = (pair.0.loc.0 - dx, pair.0.loc.1 - dy);
                if self.bounds(an1) && !self.antinodes.contains(&an1) {self.antinodes.push(an1);}
                let an2 = (pair.1.loc.0 + dx, pair.1.loc.1 + dy);
                if self.bounds(an2) && !self.antinodes.contains(&an2) {self.antinodes.push(an2);}
            }
        }
    }

    fn place_antinodes_2(&mut self) {
        let chars = self.chars_used();
        for c in chars.iter() {
            let c_pairs = self.pairs(*c);
            for pair in c_pairs.iter() {
                let dx = pair.1.loc.0 - pair.0.loc.0;
                let dy = pair.1.loc.1 - pair.0.loc.1;
                let mut an1 = pair.0.loc;
                if !self.antinodes.contains(&an1) {self.antinodes.push(an1);}
                loop {
                    an1 = (an1.0 - dx, an1.1 - dy);
                    if self.bounds(an1) {
                        if !self.antinodes.contains(&an1) {self.antinodes.push(an1);}
                    } else {break;}
                }
                let mut an2 = pair.1.loc;
                if !self.antinodes.contains(&an2) {self.antinodes.push(an2);}
                loop {
                    an2 = (an2.0 + dx, an2.1 + dy);
                    if self.bounds(an2) {
                        if !self.antinodes.contains(&an2) {self.antinodes.push(an2);}
                    } else {break;}
                }
            }
        }
    }

    fn bounds(&self, coords: (i32, i32)) -> bool {
        if coords.0 < 0 {return false;}
        if coords.0 >= self.height {return false;}
        if coords.1 < 0 {return false;}
        if coords.1 >= self.width {return false;}
        true
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
struct Signal {
    loc: (i32, i32),
    c: char
}

impl Signal {
    #![allow(dead_code)]
    fn from(x: i32, y: i32, c: char) -> Self {
        Self {
            loc: (x, y),
            c
        }
    }
}

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

fn part1(s: &str) -> usize {
    let mut m = Map::from_str(s);
    m.place_antinodes_1();
    m.antinodes.len()
}

fn part2(s: &str) -> usize {
    let mut m = Map::from_str(s);
    m.place_antinodes_2();
    m.antinodes.len()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[should_panic]
    fn map_from_str_err() {
        Map::from_str("..abc.
......
1234567
......
");
    }

    #[test]
    fn map_from_str() {
        let m = Map::from_str("..abc.
......
123456
......
");
        assert_eq!(m.signals, vec![Signal::from(0, 2, 'a'), Signal::from(0, 3, 'b'), Signal::from(0, 4, 'c'), 
Signal::from(2, 0, '1'), Signal::from(2, 1, '2'), Signal::from(2, 2, '3'), Signal::from(2, 3, '4'),
Signal::from(2, 4, '5'), Signal::from(2, 5, '6')]);
        assert_eq!(m.height, 4);
        assert_eq!(m.width, 6);
    }

    #[test]
    fn pairs_() {
        let m = Map::from_str("..abc.
..a..a
......
.a....
");
        let pairs = m.pairs('a');
        assert_eq!(pairs.len(), 6);
        assert!(pairs.contains(&(Signal::from(0, 2, 'a'), Signal::from(1, 2, 'a'))) || pairs.contains(&(Signal::from(1, 2, 'a'), Signal::from(0, 2, 'a'))));
        assert!(pairs.contains(&(Signal::from(0, 2, 'a'), Signal::from(1, 5, 'a'))) || pairs.contains(&(Signal::from(1, 5, 'a'), Signal::from(0, 2, 'a'))));
        assert!(pairs.contains(&(Signal::from(0, 2, 'a'), Signal::from(3, 1, 'a'))) || pairs.contains(&(Signal::from(3, 1, 'a'), Signal::from(0, 2, 'a'))));
        assert!(pairs.contains(&(Signal::from(1, 2, 'a'), Signal::from(1, 5, 'a'))) || pairs.contains(&(Signal::from(1, 5, 'a'), Signal::from(1, 2, 'a'))));
        assert!(pairs.contains(&(Signal::from(1, 2, 'a'), Signal::from(3, 1, 'a'))) || pairs.contains(&(Signal::from(3, 1, 'a'), Signal::from(1, 2, 'a'))));
        assert!(pairs.contains(&(Signal::from(1, 5, 'a'), Signal::from(3, 1, 'a'))) || pairs.contains(&(Signal::from(3, 1, 'a'), Signal::from(1, 5, 'a'))));
    }

    #[test]
    fn part1_1() {
        let mut m = Map::from_str("...b..
..a.ab
...ab.
......
");
        m.place_antinodes_1();
        assert_eq!(m.antinodes.len(), 6);
        assert!(m.antinodes.contains(&(0, 1)));
        assert!(m.antinodes.contains(&(0, 5)));
        assert!(m.antinodes.contains(&(1, 0)));
        assert!(m.antinodes.contains(&(3, 2)));
        assert!(m.antinodes.contains(&(3, 4)));
        assert!(m.antinodes.contains(&(3, 3)));
    }

    #[test]
    fn part1_2() {
        let mut m = Map::from_str("............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
");
        m.place_antinodes_1();
        assert_eq!(m.antinodes.len(), 14);
    }

    #[test]
    fn part2() {
        let mut m = Map::from_str("............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
");
        m.place_antinodes_2();
        assert_eq!(m.antinodes.len(), 34);
    }
}
