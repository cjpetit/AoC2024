use std::{env, fs::File, io::Read, path::Path};

#[derive(Eq, Hash, PartialEq, Debug)]
struct System {
    eq1: (u64, u64, u64),
    eq2: (u64, u64, u64),
    soln: Option<(u64, u64)>
}

impl System {
    fn new(eq1: (u64, u64, u64), eq2: (u64, u64, u64)) -> Self {
        Self {
            eq1,
            eq2,
            soln: None
        }
    }

    fn calibrate(&mut self) {
        self.eq1.2 += 10000000000000;
        self.eq2.2 += 10000000000000;
    }

    fn solve(&mut self) {
        let a_ito_b = (self.eq1.1 as f64 * -1.0 / self.eq1.0 as f64, self.eq1.2 as f64 / self.eq1.0 as f64);
        let b_coeff = self.eq2.1 as f64 + (self.eq2.0 as f64 * a_ito_b.0);
        let b_rhs = self.eq2.2 as f64 - (self.eq2.0 as f64 * a_ito_b.1);
        let b_f64 = b_rhs / b_coeff;
        let a_f64 = b_f64 * a_ito_b.0 + a_ito_b.1;
        let b = match b_f64.fract() < 0.001 || b_f64.fract() > 0.999 {
            true => b_f64.round() as u64,
            false => {
                // println!("Eq: {:?}\nBad soln: {}, {}", *self, a_f64, b_f64);
                return;}
        };
        let a = match a_f64.fract() < 0.001 || a_f64.fract() > 0.999 {
            true => a_f64.round() as u64,
            false => {
                // println!("Bad soln: {}, {}", a_f64, b_f64);
                return;}
        };
        self.soln = Some((a, b));
    }

    fn check_soln(&self) -> Result<(), ()> {
        if let Some((a, b)) = self.soln {
            if self.eq1.0 * a + self.eq1.1 * b != self.eq1.2 || self.eq2.0 * a + self.eq2.1 * b != self.eq2.2 {
                return Err(())
            }
        }
        Ok(())
    }

    fn tokens(&self) -> u64 {
        if let Some((a, b)) = self.soln {
            3*a + b
        } else {0}
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

fn part1(s: &str) -> u64 {
    let mut tokens = 0;
    let mut systems = read_systems(s);
    for sys in systems.iter_mut() {
        sys.solve();
        sys.check_soln().expect("errant solution");
        tokens += sys.tokens();
    }
    tokens
}

fn part2(s: &str) -> u64 {
    let mut tokens = 0;
    let mut systems = read_systems(s);
    for sys in systems.iter_mut() {
        sys.calibrate();
        sys.solve();
        sys.check_soln().expect("errant solution");
        tokens += sys.tokens();
    }
    tokens
}

fn read_systems(s: &str) -> Vec<System> {
    let mut systems: Vec<System> = Vec::new();
    let line_iter = s.trim().split("\n").map(|l| l.trim());
    // println!("{:?}", line_iter.clone().collect::<Vec<&str>>());
    let mut line_iter = line_iter.filter(|&l| l != "");
    // println!("{:?}", line_iter.clone().collect::<Vec<&str>>());
    loop {
        let mut a_phrase = match line_iter.next() {
            Some(s) => s,
            None => {break;}
        };
        let mut b_phrase = line_iter.next().expect("phrase list ending after a-phrase");
        let mut prize_phrase = line_iter.next().expect("phrase list ending after b-phrase");

        a_phrase = &a_phrase[a_phrase.find('+').unwrap()+1..];
        let a1_str = &a_phrase[..a_phrase.find(',').unwrap()];
        let a2_str = &a_phrase[a_phrase.find('+').unwrap()+1..];
        let a1: u64 = a1_str.parse().expect("parse error");
        let a2: u64 = a2_str.parse().expect("parse error");

        b_phrase = &b_phrase[b_phrase.find('+').unwrap()+1..];
        let b1_str = &b_phrase[..b_phrase.find(',').unwrap()];
        let b2_str = &b_phrase[b_phrase.find('+').unwrap()+1..];
        let b1: u64 = b1_str.parse().expect("parse error");
        let b2: u64 = b2_str.parse().expect("parse error");

        prize_phrase = &prize_phrase[prize_phrase.find('=').unwrap()+1..];
        let prize1_str = &prize_phrase[..prize_phrase.find(',').unwrap()];
        let prize2_str = &prize_phrase[prize_phrase.find('=').unwrap()+1..];
        let prize1: u64 = prize1_str.parse().expect("parse error");
        let prize2: u64 = prize2_str.parse().expect("parse error");

        systems.push(System::new((a1, b1, prize1), (a2, b2, prize2)));
    }

    systems
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solve_good() {
        let mut sys = System::new((94, 22, 8400), (34, 67, 5400));
        sys.solve();
        let expected = Some((80, 40));
        assert_eq!(sys.soln, expected);
    }

    #[test]
    fn solve_bad() {
        let mut sys = System::new((26, 67, 12748), (66, 21, 12176));
        sys.solve();
        assert_eq!(sys.soln, None);
    }

    #[test]
    fn read_systems_() {
        let systems = read_systems("Button A: X+99, Y+37
Button B: X+18, Y+26
Prize: X=9441, Y=5051

Button A: X+32, Y+49
Button B: X+39, Y+13
Prize: X=19007, Y=19244

Button A: X+89, Y+60
Button B: X+13, Y+73
Prize: X=4445, Y=4731
");
        let expected = Vec::from([System::new((99, 18, 9441), (37, 26, 5051)),
            System::new((32, 39, 19007), (49, 13, 19244)), System::new((89, 13, 4445), (60, 73, 4731))]);
        assert_eq!(systems, expected);
    }

    #[test]
    fn part1_() {
        let tokens = part1("Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
");
        assert_eq!(tokens, 480);
    }

    #[test]
    fn calibrate() {
        let mut tokens = read_systems("Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
");
        for token in tokens.iter_mut() {
            token.calibrate();
            token.solve();
        }
        let mut token_iter = tokens.iter();
        assert_eq!(token_iter.next().unwrap().soln, None);
        assert!(token_iter.next().unwrap().soln.is_some());
        assert_eq!(token_iter.next().unwrap().soln, None);
        assert!(token_iter.next().unwrap().soln.is_some());
    }
}