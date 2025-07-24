use std::{env, fs::File, io::Read, path::Path};

#[derive(Debug, Hash, PartialEq, Eq)]
struct Robot {
    loc: (i16, i16),
    vel: (i16, i16)
}

impl Robot {
    fn new(loc: (i16, i16), vel: (i16, i16)) -> Self {
        Self {
            loc,
            vel
        }
    }

    fn elapse(&mut self) {
        self.loc.0 = (self.loc.0 + self.vel.0 + 101) % 101;
        self.loc.1 = (self.loc.1 + self.vel.1 + 103) % 103;
    }

    fn quadrant(&self) -> Option<u8> {
        if self.loc.0 < 50 && self.loc.1 < 51 {Some(0)}
        else if self.loc.0 < 50 && self.loc.1 > 51 {Some(1)}
        else if self.loc.0 > 50 && self.loc.1 > 51 {Some(2)}
        else if self.loc.0 > 50 && self.loc.1 < 51 {Some(3)}
        else {None}
    }

    fn has_neighbor(&self, robots: &Vec<Robot>) -> bool {
        let loc_vec = to_loc_vec(robots);
        // for loc in [(1, 2), (3, 4)].iter() {
        //     if loc_vec.contains(&loc) {
        //         return true;
        //     }
        // }
        // false
        let (x, y) = self.loc;
        let neighbors = [(x-1,y-1),(x-1,y),(x-1,y+1),(x,y+1),(x+1,y+1),(x+1,y),(x+1,y-1),(x,y-1)];
        !neighbors.iter().filter(|loc| loc_vec.contains(loc)).collect::<Vec<_>>().is_empty()
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
    let mut robots = read_robots(s);
    for _ in 0..100 {
        for robot in robots.iter_mut() {
            robot.elapse();
        }
        // display(&robots);
    }
    safety_factor(&robots)
}

fn part2(s: &str) -> u32 {
    let mut robots = read_robots(s);
    for n in 0..10000 {
        if neighbor_ratio(&robots) > 0.7 {
            return n;
        }
        for robot in robots.iter_mut() {
            robot.elapse();
        }
    }
    panic!("christmas tree not found");
}

fn read_robots(s: &str) -> Vec<Robot> {
    let mut robots: Vec<Robot> = vec![];
    for desc in s.trim().split("\n").map(|d| d.trim()) {
        let desc = &desc[desc.find('=').unwrap()+1..];
        let coords = &desc[..desc.find(' ').unwrap()];
        let vel_comps = &desc[desc.find('=').unwrap()+1..];
        let mut coord_iter = coords.split(",");
        let x_coord: i16 = coord_iter.next().unwrap().parse().unwrap();
        let y_coord: i16 = coord_iter.next().unwrap().parse().unwrap();
        assert!(coord_iter.next().is_none());
        let mut vel_iter = vel_comps.split(",");
        let x_vel: i16 = vel_iter.next().unwrap().parse().unwrap();
        let y_vel: i16 = vel_iter.next().unwrap().parse().unwrap();
        assert!(vel_iter.next().is_none());
        robots.push(Robot::new((x_coord, y_coord), (x_vel, y_vel)));
    }
    robots
}

fn safety_factor(robots: &Vec<Robot>) -> u64 {
    let mut q0 = 0;
    let mut q1 = 0;
    let mut q2 = 0;
    let mut q3 = 0;
    for robot in robots.iter() {
        match robot.quadrant() {
            Some(0) => {q0 += 1;},
            Some(1) => {q1 += 1;},
            Some(2) => {q2 += 1;},
            Some(3) => {q3 += 1;},
            _ => {}
        }
    }
    q0*q1*q2*q3
}

#[allow(dead_code)]
fn display(robots: &Vec<Robot>) {
    let loc_vec = to_loc_vec(robots);
    for _ in 0..103 {
        print!("O");
    }
    print!("\n");
    for j in 0..103 {
        print!("O");
        for i in 0..101 {
            if loc_vec.contains(&(i, j)) {
                print!("X")
            } else {print!(" ");}
        }
        print!("O\n");
    }
    for _ in 0..103 {
        print!("O");
    }
    print!("\n\n");
}

fn neighbor_ratio(robots: &Vec<Robot>) -> f64 {
    let mut neighbors = 0.0;
    for robot in robots.iter() {
        if robot.has_neighbor(robots) {neighbors += 1.0;}
    }
    neighbors / robots.len() as f64
}

fn to_loc_vec(robots: &Vec<Robot>) -> Vec<(i16, i16)> {
    let mut loc_vec: Vec<(i16, i16)> = vec![];
    for robot in robots.iter() {
        loc_vec.push(robot.loc);
    }
    loc_vec
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn read_robots_() {
        let robots = read_robots("p=84,26 v=99,-23
p=98,17 v=-41,-46
p=3,84 v=-17,-23
");
        let set: HashSet<(i16, i16)> = HashSet::from_iter(robots.iter().map(|r| r.loc));
        let expected: HashSet<(i16, i16)> = HashSet::from([(84, 26), (98, 17), (3, 84)]);
        assert_eq!(set, expected);
    }

    #[test]
    fn elapse_3() {
        let mut robots = vec![Robot::new((84, 26), (99, -23)), Robot::new((98, 17), (-41, -46)), Robot::new((3, 84), (17, -23))];
        for robot in robots.iter_mut() {
            robot.elapse();
            robot.elapse();
            robot.elapse();
        }
        let set: HashSet<(i16, i16)> = HashSet::from_iter(robots.iter().map(|r| r.loc));
        let expected: HashSet<(i16, i16)> = HashSet::from([(78, 60), (76, 85), (54, 15)]);
        assert_eq!(set, expected);
    }

    #[test]
    fn safety_factor_() {
        let robots = vec![Robot::new((84, 26), (99, -23)), Robot::new((18, 17), (-41, -46)), Robot::new((3, 44), (17, -23)), Robot::new((3, 84), (17, -23)), Robot::new((93, 84), (17, -23))];
        assert_eq!(safety_factor(&robots), 2);
    }
}
