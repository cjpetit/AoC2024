use std::{cmp::min, collections::{HashMap, HashSet}, env, fs::File, io::Read, path::Path};

#[derive(PartialEq, Debug, Clone)]
struct Maze {
    map: HashMap<(usize, usize), Square>,
    width: usize,
    height: usize,
    start: (usize, usize),
    end: (usize, usize),
    finishers: Vec<Reindeer>,
    paths: HashSet<(usize, usize)>
}

impl Maze {
    fn new(map: HashMap<(usize, usize), Square>, width: usize, height: usize, start: (usize, usize), end: (usize, usize)) -> Self {
        Self {
            map,
            width,
            height,
            start,
            end,
            finishers: vec![],
            paths: HashSet::new()
        }
    }

    fn solve(&mut self) {
        let mut deer = vec![Reindeer::new(self.start)];
        while !deer.is_empty() {
            let mut new_deer = Vec::<Reindeer>::new();
            loop {
                if let Some(mut d) = deer.pop() {
                    let (result, v) = d.advance(self);
                    if result {
                        new_deer.push(d);
                        new_deer.extend(v);
                    }
                } else {break;}
            }
            deer = new_deer;
        }
        for finisher in self.finishers.iter() {
            if finisher.score == self.score() {
                self.paths = self.paths.union(&finisher.path).map(|&p| p).collect()
            }
        }
    }

    fn score(&self) -> u32 {
        self.map.get(&self.end).unwrap().score().expect("no score found for end square")
    }

    fn update_square(&mut self, deer: &Reindeer) -> bool {
        if deer.loc == self.end {
            self.finishers.push(deer.clone());
        }
        let sq = self.map.get_mut(&deer.loc).unwrap();
        match deer.dir {
            '>' => {
                if let Some(prev) = sq.r {
                    if deer.score <= prev {
                        sq.r = Some(deer.score);
                        true
                    } else {false}
                } else {
                    sq.r = Some(deer.score);
                    true
                }
            },
            'v' => {
                if let Some(prev) = sq.d {
                    if deer.score <= prev {
                        sq.d = Some(deer.score);
                        true
                    } else {false}
                } else {
                    sq.d = Some(deer.score);
                    true
                }
            },
            '<' => {
                if let Some(prev) = sq.l {
                    if deer.score <= prev {
                        sq.l = Some(deer.score);
                        true
                    } else {false}
                } else {
                    sq.l = Some(deer.score);
                    true
                }
            },
            '^' => {
                if let Some(prev) = sq.u {
                    if deer.score <= prev {
                        sq.u = Some(deer.score);
                        true
                    } else {false}
                } else {
                    sq.u = Some(deer.score);
                    true
                }
            },
            _ => {panic!();}
        }
    }

    #[allow(dead_code)]
    fn display(&self) {
        for i in 0..self.height {
            print!("\n");
            for j in 0..self.width {
                if !self.map.get(&(i, j)).expect("no square at expected index").open {print!("#");}
                else if self.paths.contains(&(i, j)) {print!("O");}
                else {print!(".");}
            }
        }
        print!("\n");
    }
}

#[derive(PartialEq, Debug, Clone)]
struct Reindeer {
    loc: (usize, usize),
    dir: char,
    score: u32,
    path: HashSet<(usize, usize)>
}

impl Reindeer {
    fn new(loc: (usize, usize)) -> Self {
        Self {
            loc,
            dir: '>',
            score: 0,
            path: HashSet::from([loc])
        }
    }

    fn advance(&mut self, maze: &mut Maze) -> (bool, Vec<Reindeer>) {
        // println!("solving from ({}, {}, {})", self.loc.0, self.loc.1, self.dir);
        if !maze.update_square(&self) {return (false, vec![]);}
        match (maze.map.get(&self.in_front()).unwrap().open, maze.map.get(&self.to_left()).unwrap().open, maze.map.get(&self.to_right()).unwrap().open) {
            (false, false, false) => (false, vec![]),
            (true, false, false) => {
                self.move_forward();
                (true, vec![])
            },
            (false, true, false) => {
                self.turn_left();
                self.move_forward();
                (true, vec![])
            },
            (false, false, true) => {
                self.turn_right();
                self.move_forward();
                (true, vec![])
            },
            (true, true, false) => {
                let mut left_deer = self.clone();
                self.move_forward();
                left_deer.turn_left();
                left_deer.move_forward();
                (true, vec![left_deer])
            },
            (true, false, true) => {
                let mut right_deer = self.clone();
                self.move_forward();
                right_deer.turn_right();
                right_deer.move_forward();
                (true, vec![right_deer])
            },
            (false, true, true) => {
                let mut left_deer = self.clone();
                self.turn_right();
                self.move_forward();
                left_deer.turn_left();
                left_deer.move_forward();
                (true, vec![left_deer])
            },
            (true, true, true) => {
                let mut right_deer = self.clone();
                let mut left_deer = self.clone();
                self.move_forward();
                right_deer.turn_right();
                right_deer.move_forward();
                left_deer.turn_left();
                left_deer.move_forward();
                (true, vec![right_deer, left_deer])
            }
        }
    }

    fn in_front(&self) -> (usize, usize) {
        match self.dir {
            '>' => (self.loc.0, self.loc.1 + 1),
            'v' => (self.loc.0 + 1, self.loc.1),
            '<' => (self.loc.0, self.loc.1 - 1),
            '^' => (self.loc.0 - 1, self.loc.1),
            _ => {panic!();}
        }
    }

    fn to_left(&self) -> (usize, usize) {
        match self.dir {
            'v' => (self.loc.0, self.loc.1 + 1),
            '<' => (self.loc.0 + 1, self.loc.1),
            '^' => (self.loc.0, self.loc.1 - 1),
            '>' => (self.loc.0 - 1, self.loc.1),
            _ => {panic!();}
        }
    }

    fn to_right(&self) -> (usize, usize) {
        match self.dir {
            '^' => (self.loc.0, self.loc.1 + 1),
            '>' => (self.loc.0 + 1, self.loc.1),
            'v' => (self.loc.0, self.loc.1 - 1),
            '<' => (self.loc.0 - 1, self.loc.1),
            _ => {panic!();}
        }
    }

    fn move_forward(&mut self) {
        self.loc = match self.dir {
            '>' => (self.loc.0, self.loc.1+1),
            '^' => (self.loc.0-1, self.loc.1),
            '<' => (self.loc.0, self.loc.1-1),
            'v' => (self.loc.0+1, self.loc.1),
            unrec => {panic!("unrecognized char: {}", unrec);}
        };
        self.score += 1;
        self.path.insert(self.loc);
    }

    fn turn_left(&mut self) {
        self.dir = match self.dir {
            '>' => '^',
            '^' => '<',
            '<' => 'v',
            'v' => '>',
            unrec => {panic!("unrecognized char: {}", unrec);}
        };
        self.score += 1000;
    }

    fn turn_right(&mut self) {
        self.dir = match self.dir {
            '>' => 'v',
            '^' => '>',
            '<' => '^',
            'v' => '<',
            unrec => {panic!("unrecognized char: {}", unrec);}
        };
        self.score += 1000;
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
struct Square {
    open: bool,
    l: Option<u32>,
    r: Option<u32>,
    u: Option<u32>,
    d: Option<u32>
}

impl Square {
    fn new(open: bool) -> Self {
        Self {
            open,
            l: None,
            r: None,
            u: None,
            d: None
        }
    }

    fn score(&self) -> Option<u32> {
        let mut score = Option::<u32>::None;
        if let Some(ls) = self.l {
            score = match score {
                None => Some(ls),
                Some(prev) => Some(min(prev, ls))
            }
        }
        if let Some(rs) = self.r {
            score = match score {
                None => Some(rs),
                Some(prev) => Some(min(prev, rs))
            }
        }
        if let Some(us) = self.u {
            score = match score {
                None => Some(us),
                Some(prev) => Some(min(prev, us))
            }
        }
        if let Some(ds) = self.d {
            score = match score {
                None => Some(ds),
                Some(prev) => Some(min(prev, ds))
            }
        }
        score
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

fn part1(s: &str) -> u32 {
    let mut maze = read_maze(s);
    maze.solve();
    maze.score()
}

fn part2(s: &str) -> usize {
    let mut maze = read_maze(s);
    maze.solve();
    maze.paths.len()
}

fn read_maze(s: &str) -> Maze {
    let mut map = HashMap::<(usize, usize), Square>::new();
    let mut start = Option::<(usize, usize)>::None;
    let mut end = Option::<(usize, usize)>::None;
    let line_iter = s.trim().split("\n").map(|l| l.trim());
    let height = line_iter.clone().collect::<Vec<_>>().len();
    let mut width = Option::<usize>::None;
    for (i, line) in line_iter.enumerate() {
        let chars = line.chars();
        let row_width = chars.clone().collect::<Vec<_>>().len();
        if let None = width { width = Some(row_width);}
        else {assert_eq!(width, Some(row_width));}
        for (j, c) in chars.enumerate() {
            let open = match c {
                '#' => false,
                '.' => true,
                'S' => {
                    assert!(start.is_none());
                    start = Some((i, j));
                    true
                },
                'E' => {
                    assert!(end.is_none());
                    end = Some((i, j));
                    true
                },
                unrec => {panic!("unrecognized char: {}", unrec);}
            };
            map.insert((i, j), Square::new(open));
        }
    }
    Maze::new(map, width.unwrap(), height, start.expect("start not found"), end.expect("end not found"))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn read_maze_() {
        let maze = read_maze("###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############
");
        assert_eq!(maze.map.get(&(0, 0)).unwrap().open, false);
        assert_eq!(maze.map.get(&(1, 1)).unwrap().open, true);
        assert_eq!(maze.map.get(&(1, 13)).unwrap().open, true);
        assert_eq!(maze.map.get(&(13, 1)).unwrap().open, true);
        assert_eq!(maze.start, (13, 1));
        assert_eq!(maze.end, (1, 13));
        assert_eq!(maze.width, 15);
        assert_eq!(maze.height, 15);
    }

    #[test]
    fn part1_() {
        let s = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############
";
        assert_eq!(part1(s), 7036);
    }

    #[test]
    fn paths() {
        let mut maze = read_maze("###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############
");
        maze.solve();
        maze.display();
        assert_eq!(maze.paths.len(), 45);
    }
}
