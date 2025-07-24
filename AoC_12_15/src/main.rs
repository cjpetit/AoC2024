use std::{collections::{HashMap, VecDeque}, env, fs::File, io::Read, path::Path};

#[derive(PartialEq, Debug)]
struct Map {
    height: usize,
    width: usize,
    conts: HashMap<(usize, usize), MapCont>,
    robot: (usize, usize),
    moves: VecDeque<char>
}

impl Map {
    fn new(height: usize, width: usize, conts: HashMap<(usize, usize), MapCont>, robot: (usize, usize), moves: VecDeque<char>) -> Self {
        Self {
            height,
            width,
            conts,
            robot,
            moves
        }
    }

    fn apply_moves(&mut self) {
        loop {
            if let Some(dir) = self.moves.pop_front() {
                self.try_move(self.robot, dir);
            } else {break;}
        }
    }

    fn try_move(&mut self, from: (usize, usize), dir: char) -> bool {
        use MapCont::*;
        let dest = match dir {
            '^' => (from.0 - 1, from.1),
            '>' => (from.0, from.1 + 1),
            'v' => (from.0 + 1, from.1),
            '<' => (from.0, from.1 - 1),
            c => {panic!("unexpected char in moves: {}", c);}
        };
        let dest_cont = self.conts.get(&dest).expect("move location not in hashmap");
        let from_cont = self.conts.get(&from).expect("src location not in hashmap").clone();
        match dest_cont {
            Blank => {
                *self.conts.get_mut(&dest).unwrap() = from_cont;
                *self.conts.get_mut(&from).unwrap() = Blank;
                if from_cont == Robot {self.robot = dest;}
                true
            },
            Wall => {false},
            Box => {
                if self.try_move(dest, dir) {
                    assert!(self.try_move(from, dir));
                    true
                } else {false}
            },
            Robot => {panic!("try_move call into robot loc");},
            LeftBox => {
                if dir == '>' || dir == '<' {
                    if self.try_move(dest, dir) {
                        assert!(self.try_move(from, dir));
                        true
                    } else {false}
                } else if self.can_move((dest.0, dest.1+1), dir) {
                    if self.try_move(dest, dir) {
                        assert!(self.try_move((dest.0, dest.1+1), dir));
                        assert!(self.try_move(from, dir));
                        true
                    } else {false}
                } else {false}
            },
            RightBox => {
                if dir == '>' || dir == '<' {
                    if self.try_move(dest, dir) {
                        assert!(self.try_move(from, dir));
                        true
                    } else {false}
                } else if self.can_move((dest.0, dest.1-1), dir) {
                    if self.try_move(dest, dir) {
                        assert!(self.try_move((dest.0, dest.1-1), dir));
                        assert!(self.try_move(from, dir));
                        true
                    } else {false}
                } else {false}
            }
        }
    }

    fn can_move(&mut self, from: (usize, usize), dir: char) -> bool {
        use MapCont::*;
        let dest = match dir {
            '^' => (from.0 - 1, from.1),
            '>' => (from.0, from.1 + 1),
            'v' => (from.0 + 1, from.1),
            '<' => (from.0, from.1 - 1),
            c => {panic!("unexpected char in moves: {}", c);}
        };
        let dest_cont = self.conts.get(&dest).expect("move location not in hashmap");
        match dest_cont {
            Blank => {true},
            Wall => {false},
            Box => {self.can_move(dest, dir)},
            Robot => {panic!("try_move call into robot loc");},
            LeftBox => {
                if dir == '>' || dir == '<' {
                    self.can_move(dest, dir)
                } else if self.can_move((dest.0, dest.1+1), dir) {
                    self.can_move(dest, dir)
                } else {false}
            },
            RightBox => {
                if dir == '>' || dir == '<' {
                    self.can_move(dest, dir)
                } else if self.can_move((dest.0, dest.1-1), dir) {
                    self.can_move(dest, dir)
                } else {false}
            }
        }
    }

    fn gps_sum(&self) -> usize {
        let mut sum = 0;
        for ((i, j), cont) in self.conts.iter() {
            match *cont {
                MapCont::Box => {sum += 100 * i + j;}
                MapCont::LeftBox => {sum += 100 * i + j;},
                _ => {}
            }
        }
        sum
    }

    #[allow(dead_code)]
    fn display(&self) {
        use MapCont::*;
        print!("\n");
        for i in 0..self.height {
            print!("\n");
            for j in 0..self.width {
                let cont = self.conts.get(&(i, j)).unwrap();
                let c = match *cont {
                    Blank => '.',
                    Robot => '@',
                    Wall => '#',
                    Box => 'O',
                    LeftBox => '[',
                    RightBox => ']'
                };
                print!("{}", c);
            }
        }
    }
}

#[derive(PartialEq, Debug, Copy, Clone)]
enum MapCont {
    Wall, Box, Robot, Blank,
    LeftBox, RightBox
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
    let mut map = read_to_map(s);
    map.apply_moves();
    map.gps_sum()
}

fn part2(s: &str) -> usize {
    let mut map = read_wide(s);
    map.apply_moves();
    map.gps_sum()
}

fn read_to_map(s: &str) -> Map {
    let mut line_iter = s.trim().split("\n").map(|l| l.trim()).enumerate();
    let mut hmap: HashMap<(usize, usize), MapCont> = HashMap::new();
    // let mut height = 0;
    let width = line_iter.clone().next().unwrap().1.len();
    let mut robot_loc: Option<(usize, usize)> = None;
    let height = loop {
        let (i, line) = line_iter.next().expect("end of lines in first loop");
        if line == "" {
            break i;
        }
        assert_eq!(width, line.len());
        for (j, c) in line.chars().enumerate() {
            let cont = match c {
                '#' => MapCont::Wall,
                'O' => MapCont::Box,
                '@' => {
                    assert!(robot_loc.is_none());
                    robot_loc = Some((i, j));
                    MapCont::Robot
                },
                '.' => MapCont::Blank,
                x => {panic!("unexpected char in map: {}", x);}
            };
            assert!(hmap.insert((i, j), cont).is_none());
        }
    };

    let mut moves = VecDeque::<char>::new();
    loop {
        if let Some((_,line)) = line_iter.next() {
            for c in line.chars() {
                moves.push_back(c);
            }
        } else {break;}
    }

    assert!(height > 0);
    Map::new(height, width, hmap, robot_loc.expect("robot not found"), moves)
}

fn read_wide(s: &str) -> Map {
    let mut line_iter = s.trim().split("\n").map(|l| l.trim()).enumerate();
    let mut hmap: HashMap<(usize, usize), MapCont> = HashMap::new();
    // let mut height = 0;
    let width = line_iter.clone().next().unwrap().1.len() * 2;
    let mut robot_loc: Option<(usize, usize)> = None;
    let height = loop {
        let (i, line) = line_iter.next().expect("end of lines in first loop");
        if line == "" {
            break i;
        }
        assert_eq!(width, line.len() * 2);
        for (j, c) in line.chars().enumerate() {
            match c {
                '#' => {
                    assert!(hmap.insert((i, j*2), MapCont::Wall).is_none());
                    assert!(hmap.insert((i, j*2+1), MapCont::Wall).is_none());
                },
                'O' => {
                    assert!(hmap.insert((i, j*2), MapCont::LeftBox).is_none());
                    assert!(hmap.insert((i, j*2+1), MapCont::RightBox).is_none());
                },
                '.' => {
                    assert!(hmap.insert((i, j*2), MapCont::Blank).is_none());
                    assert!(hmap.insert((i, j*2+1), MapCont::Blank).is_none());
                },
                '@' => {
                    assert!(hmap.insert((i, j*2), MapCont::Robot).is_none());
                    assert!(robot_loc.is_none());
                    robot_loc = Some((i, j*2));
                    assert!(hmap.insert((i, j*2+1), MapCont::Blank).is_none());
                },
                x => {panic!("unexpected char in map: {}", x);}
            }
        }
    };

    let mut moves = VecDeque::<char>::new();
    loop {
        if let Some((_,line)) = line_iter.next() {
            for c in line.chars() {
                moves.push_back(c);
            }
        } else {break;}
    }

    assert!(height > 0);
    Map::new(height, width, hmap, robot_loc.expect("robot not found"), moves)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn read_to_map_() {
        let map = read_to_map("########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<
");
        assert_eq!(map.width, 8);
        assert_eq!(map.height, 8);
        assert_eq!(map.robot, (2, 2));
        assert_eq!(map.moves, VecDeque::from(['<','^','^','>','>','>','v','v','<','v','>','>','v','<','<']));
        assert_eq!(map.conts.get(&(0, 5)), Some(&MapCont::Wall));
        assert_eq!(map.conts.get(&(2, 2)), Some(&MapCont::Robot));
        assert_eq!(map.conts.get(&(5, 4)), Some(&MapCont::Box));
        assert_eq!(map.conts.get(&(5, 5)), Some(&MapCont::Blank));
    }

    #[test]
    fn gps_sum() {
        let map = read_to_map("##########
#.O.O.OOO#
#........#
#OO......#
#OO@.....#
#O#.....O#
#O.....OO#
#O.....OO#
#OO....OO#
##########

>
");
        assert_eq!(map.gps_sum(), 10092);
    }

    #[test]
    fn apply_moves_() {
        let mut map = read_to_map("########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<
");
        let expected = read_to_map("########
#....OO#
##.....#
#.....O#
#.#O@..#
#...O..#
#...O..#
########

>
");
        map.apply_moves();
        assert_eq!(map.conts, expected.conts);
    }

    #[test]
    fn read_wide_() {
        let map = read_wide("########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<
");
        assert_eq!(map.width, 16);
        assert_eq!(map.height, 8);
        assert_eq!(map.robot, (2, 4));
        assert_eq!(map.moves, VecDeque::from(['<','^','^','>','>','>','v','v','<','v','>','>','v','<','<']));
        assert_eq!(map.conts.get(&(0, 10)), Some(&MapCont::Wall));
        assert_eq!(map.conts.get(&(0, 11)), Some(&MapCont::Wall));
        assert_eq!(map.conts.get(&(2, 3)), Some(&MapCont::Wall));
        assert_eq!(map.conts.get(&(2, 4)), Some(&MapCont::Robot));
        assert_eq!(map.conts.get(&(2, 5)), Some(&MapCont::Blank));
        assert_eq!(map.conts.get(&(5, 8)), Some(&MapCont::LeftBox));
        assert_eq!(map.conts.get(&(5, 9)), Some(&MapCont::RightBox));
        assert_eq!(map.conts.get(&(5, 10)), Some(&MapCont::Blank));
        assert_eq!(map.conts.get(&(5, 11)), Some(&MapCont::Blank));
    }

    #[test]
    fn apply_moves_wide() {
        let mut map = read_wide("#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^
");
        map.apply_moves();
        assert_eq!(map.gps_sum(), 618);
    }
}
