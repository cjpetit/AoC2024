use std::{env, fs::File, io::Read, path::Path};

#[derive(Clone)]
struct Map {
    width: i32,
    height: i32,
    obstacles: Vec<(i32, i32)>,
    guard: Guard,
    history: Vec<((i32, i32), Direction)>
}

impl Map {
    fn from_str(s: &str) -> Self {
        let mut width = 0;
        let mut height = 0;
        let mut obstacles: Vec<(i32, i32)> = Vec::new();
        let mut guard_loc = (-1, -1);
        let mut x = 0;
        for byte in s.as_bytes() {
            match *byte as char {
                '.' => {x += 1;},
                '#' => {
                    obstacles.push((height, x));
                    x += 1;
                },
                '^' => {
                    guard_loc = (height, x);
                    x += 1;
                },
                '\n' => {
                    if width == 0 {width = x;}
                    else if width != x {panic!("Inconsistent width while scanning map")}
                    height += 1;
                    x = 0;
                },
                '\r' => {},
                _ => {panic!("Unrecognized char while scanning map");}
            }
        }

        assert_ne!(guard_loc, (-1, -1));
        Self {
            width,
            height,
            obstacles,
            guard: Guard {
                loc: guard_loc,
                dir: Direction::Up
            },
            history: vec![(guard_loc, Direction::Up)]
        }
    }

    fn advance(&mut self) -> Result<bool, ()> {
        use Direction::*;
        let (x, y) = self.guard.loc;
        match self.guard.dir {
            Up => {
                if !self.obstacles.contains(&(x-1, y)) {
                    self.guard.loc = (x-1, y);
                } else {
                    self.guard.dir = Right;
                }
                if self.history.contains(&self.guard_status()) {return Err(());}
                self.history.push(self.guard_status());
            },
            Right => {
                if !self.obstacles.contains(&(x, y+1)) {
                    self.guard.loc = (x, y+1);
                } else {
                    self.guard.dir = Down;
                }
                if self.history.contains(&self.guard_status()) {return Err(());}
                self.history.push(self.guard_status());
            },
            Down => {
                if !self.obstacles.contains(&(x+1, y)) {
                    self.guard.loc = (x+1, y);
                } else {
                    self.guard.dir = Left;
                }
                if self.history.contains(&self.guard_status()) {return Err(());}
                self.history.push(self.guard_status());
            },
            Left => {
                if !self.obstacles.contains(&(x, y-1)) {
                    self.guard.loc = (x, y-1);
                } else {
                    self.guard.dir = Up;
                }
                if self.history.contains(&self.guard_status()) {return Err(());}
                self.history.push(self.guard_status());
            },
        }
        // println!("Advanced to {:?}", self.guard.loc);
        
        if self.guard.loc.0 >= self.width || self.guard.loc.0 < 0 || self.guard.loc.1 >= self.height || self.guard.loc.1 < 0 {
            self.history.pop();
            return Ok(true);
        }
        Ok(false)
    }

    fn advance_all(&mut self) -> Result<(), ()> {
        while !self.advance()? {}
        Ok(())
    }

    fn guard_status(&self) -> ((i32, i32), Direction) {
        (self.guard.loc, self.guard.dir)
    }

    fn squares_visited(&self) -> Vec<(i32, i32)> {
        let mut visited: Vec<(i32, i32)> = Vec::new();
        for (square, _) in self.history.iter() {
            if !visited.contains(square) {
                visited.push(*square);
            }
        }
        visited
    }

    fn looping_obstacles(&self) -> Vec<(i32, i32)> {
        let mut obsts: Vec<(i32, i32)> = Vec::new();
        for x in 0..self.height {
            for y in 0..self.width {
                if !self.obstacles.contains(&(x, y)) {
                    println!("checking {:?}", (x, y));
                    let mut modified = self.clone();
                    modified.obstacles.push((x, y));
                    if let Err(()) = modified.advance_all() {
                        obsts.push((x, y));
                    }
                }
            }
        }
        obsts
    }

    // fn find_all_loops(&self) -> Vec<(i32, i32)> {                        // This is an abandoned method of finding obstacle locations
    //     let mut loops: Vec<(i32, i32)> = Vec::new();                     // that would cause loops. It could only find simple square
    //     for obstacle in self.obstacles.iter() {                          // loops. It seems there is no limit to how complex a loop could
    //         if let Some(point) = self.find_as_top(*obstacle) {           // be theoretically, which is why I abandoned the
    //             if !loops.contains(&point) {loops.push(point);}          // pattern-recognition approach.
    //         }
    //         if let Some(point) = self.find_as_right(*obstacle) {
    //             if !loops.contains(&point) {loops.push(point);}
    //         }
    //         if let Some(point) = self.find_as_bottom(*obstacle) {
    //             if !loops.contains(&point) {loops.push(point);}
    //         }
    //         if let Some(point) = self.find_as_left(*obstacle) {
    //             if !loops.contains(&point) {loops.push(point);}
    //         }
    //     }
    //     loops
    // }

    // fn find_as_top(&self, obstacle: (i32, i32)) -> Option<(i32, i32)> {
    //     let (x, y) = obstacle;
    //     if x < self.height - 2 && y >= 1 && y < self.width - 1 {
    //         let mut right: Option<(i32, i32)> = None;
    //         let mut left: Option<(i32, i32)> = None;
    //         for candidate in self.obstacles.iter() {
    //             if candidate.0 == x + 1 && candidate.1 > y {
    //                 if let Some(other) = right {
    //                     if candidate.1 < other.1 {
    //                         right = Some(*candidate);
    //                     }
    //                 } else {
    //                     right = Some(*candidate);
    //                 }
    //             }
    //             if candidate.1 == y - 1 && candidate.0 > x {
    //                 if let Some(other) = left {
    //                     if candidate.0 < other.0 {
    //                         left = Some(*candidate);
    //                     }
    //                 } else {
    //                     left = Some(*candidate);
    //                 }
    //             }
    //         }
    //         if let (Some(r), Some(l)) = (right, left) {
    //             let b = (l.0 + 1, r.1 - 1);
    //             if self.is_unobstructed_loop(obstacle, r, b, l) {
    //                 return Some(b);
    //             }
    //         }
    //     }
    //     None
    // }

    // fn find_as_right(&self, obstacle: (i32, i32)) -> Option<(i32, i32)> {
    //     let (x, y) = obstacle;
    //     if y >= 2 && x >= 1 && x < self.height - 1 {
    //         let mut top: Option<(i32, i32)> = None;
    //         let mut bottom: Option<(i32, i32)> = None;
    //         for candidate in self.obstacles.iter() {
    //             if candidate.0 == x - 1 && candidate.1 < y {
    //                 if let Some(other) = top {
    //                     if candidate.1 > other.1 {
    //                         top = Some(*candidate);
    //                     }
    //                 } else {
    //                     top = Some(*candidate);
    //                 }
    //             }
    //             if candidate.1 == y - 1 && candidate.0 > x {
    //                 if let Some(other) = bottom {
    //                     if candidate.0 < other.0 {
    //                         bottom = Some(*candidate);
    //                     }
    //                 } else {
    //                     bottom = Some(*candidate);
    //                 }
    //             }
    //         }
    //         if let (Some(t), Some(b)) = (top, bottom) {
    //             let l = (b.0 - 1, t.1 - 1);
    //             if self.is_unobstructed_loop(t, obstacle, b, l) {
    //                 return Some(l);
    //             }
    //         }
    //     }
    //     None
    // }

    // fn find_as_bottom(&self, obstacle: (i32, i32)) -> Option<(i32, i32)> {
    //     let (x, y) = obstacle;
    //     if x >= 2 && y >= 1 && y < self.width - 1 {
    //         let mut right: Option<(i32, i32)> = None;
    //         let mut left: Option<(i32, i32)> = None;
    //         for candidate in self.obstacles.iter() {
    //             if candidate.1 == y + 1 && candidate.0 < x {
    //                 if let Some(other) = right {
    //                     if candidate.0 > other.0 {
    //                         right = Some(*candidate);
    //                     }
    //                 } else {
    //                     right = Some(*candidate);
    //                 }
    //             }
    //             if candidate.0 == x - 1 && candidate.1 < y {
    //                 if let Some(other) = left {
    //                     if candidate.1 > other.1 {
    //                         left = Some(*candidate);
    //                     }
    //                 } else {
    //                     left = Some(*candidate);
    //                 }
    //             }
    //         }
    //         if let (Some(r), Some(l)) = (right, left) {
    //             let t = (r.0 - 1, l.1 + 1);
    //             if self.is_unobstructed_loop(t, r, obstacle, l) {
    //                 return Some(t);
    //             }
    //         }
    //     }
    //     None
    // }

    // fn find_as_left(&self, obstacle: (i32, i32)) -> Option<(i32, i32)> {
    //     let (x, y) = obstacle;
    //     if y < self.width - 2 && x >= 1 && x < self.height - 1 {
    //         let mut top: Option<(i32, i32)> = None;
    //         let mut bottom: Option<(i32, i32)> = None;
    //         for candidate in self.obstacles.iter() {
    //             if candidate.1 == y + 1 && candidate.0 < x {
    //                 if let Some(other) = top {
    //                     if candidate.0 > other.0 {
    //                         top = Some(*candidate);
    //                     }
    //                 } else {
    //                     top = Some(*candidate);
    //                 }
    //             }
    //             if candidate.0 == x + 1 && candidate.1 > y {
    //                 if let Some(other) = bottom {
    //                     if candidate.1 < other.1 {
    //                         bottom = Some(*candidate);
    //                     }
    //                 } else {
    //                     bottom = Some(*candidate);
    //                 }
    //             }
    //         }
    //         if let (Some(t), Some(b)) = (top, bottom) {
    //             let r = (t.0 + 1, b.1 + 1);
    //             if self.is_unobstructed_loop(t, r, b, obstacle) {
    //                 return Some(r);
    //             }
    //         }
    //     }
    //     None
    // }

    // fn is_unobstructed_loop(&self, top: (i32, i32), right: (i32, i32), bottom: (i32, i32), left: (i32, i32)) -> bool {
    //     for i in top.1..right.1 {
    //         if self.obstacles.contains(&(right.0, i)) {return false;}
    //         if self.obstacles.contains(&(left.0, i)) {return false;}
    //     }
    //     for i in right.0+1..bottom.0-1 {
    //         if self.obstacles.contains(&(i, bottom.1)) {return false;}
    //         if self.obstacles.contains(&(i, top.1)) {return false;}
    //     }
    //     true
    // }
}

#[derive(Clone)]
struct Guard {
    loc: (i32, i32),
    dir: Direction
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Direction {
    Up, Right, Down, Left
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
    m.advance_all().expect("unexpected loop in part 1");
    m.squares_visited().len()
}

fn part2(s: &str) -> usize {                    // Very ineffecient as is. Took about an hour with AoC input.
    let m = Map::from_str(s);
    m.looping_obstacles().len()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn from_str() {
        let s = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...\n";
        let m = Map::from_str(s);
        assert_eq!(m.width, 10);
        assert_eq!(m.height, 10);
        assert_eq!(m.obstacles, vec![(0, 4), (1, 9), (3, 2), (4, 7), (6, 1), (7, 8), (8, 0), (9, 6)]);
        assert_eq!(m.guard.loc, (6, 4));
    }

    #[test]
    fn advance() {
        let s = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...\n";
        let mut m = Map::from_str(s);
        m.advance().unwrap();
        assert_eq!(m.guard.loc, (5, 4));
        assert_eq!(m.guard.dir, Direction::Up);
    }

    #[test]
    fn advance_all_ok() {
        let s = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...\n";
        let mut m = Map::from_str(s);
        m.advance_all().unwrap();
        assert_eq!(m.squares_visited().len(), 41);
    }

    #[test]
    fn advance_all_err() {
        let s = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
......#.#.
#.........
......#...\n";
        let mut m = Map::from_str(s);
        if let Ok(()) = m.advance_all() {panic!();}
    }

//     #[test]
//     fn find_as_top_some() {
//         let s = "....#.....
// .........#
// ..........
// ..#.......
// .......#..
// ..........
// .#..^.....
// ........#.
// #.........
// ......#...\n";
//         let m = Map::from_str(s);
//         assert_eq!(m.find_as_top((3, 2)), Some((7, 6)));
//     }

//     #[test]
//     fn find_as_top_none() {
//         let s = "....#.....
// .........#
// ..........
// ..#.......
// .......#..
// ..........
// #...^.....
// ........#.
// #.........
// ......#...\n";
//         let m = Map::from_str(s);
//         assert_eq!(m.find_as_top((3, 2)), None);
//     }

//     #[test]
//     fn find_as_right_some() {
//         let s = "....#.....
// .........#
// ..........
// ..#.......
// .......#..
// ..........
// .#..^.....
// ........#.
// #.........
// ......#...\n";
//         let m = Map::from_str(s);
//         assert_eq!(m.find_as_right((1, 9)), Some((6, 3)));
//     }

//     #[test]
//     fn find_as_right_none() {
//         let s = "....#.....
// .........#
// ..........
// ..#.......
// .......#..
// ..........
// .#..^.....
// ........#.
// #.........
// ......#...\n";
//         let m = Map::from_str(s);
//         assert_eq!(m.find_as_right((7, 8)), None);
//     }

//     #[test]
//     fn find_as_bottom_some() {
//         let s = "....#.....
// .........#
// ..........
// ..........
// .......#..
// ..........
// .#..^.....
// ........#.
// #.........
// ......#...\n";
//         let m = Map::from_str(s);
//         assert_eq!(m.find_as_bottom((7, 8)), Some((0, 2)));
//     }

//     #[test]
//     fn find_as_bottom_none() {
//         let s = "....#.....
// .........#
// ..........
// ..#.......
// .......#..
// ..........
// .#..^.....
// ........#.
// #.........
// ......#...\n";
//         let m = Map::from_str(s);
//         assert_eq!(m.find_as_bottom((7, 8)), None);
//     }

//     #[test]
//     fn find_as_left_some() {
//         let s = "....#.....
// .........#
// ..........
// ..#.......
// ..........
// ..........
// .#..^.....
// ........#.
// #.........
// ......#...\n";
//         let m = Map::from_str(s);
//         assert_eq!(m.find_as_left((6, 1)), Some((4, 9)));
//     }

//     #[test]
//     fn find_as_left_none() {
//         let s = "....#.....
// .........#
// ..........
// ..#.......
// .......#..
// ..........
// .#..^.....
// ........#.
// #.........
// ......#...\n";
//         let m = Map::from_str(s);
//         assert_eq!(m.find_as_left((6, 1)), None);
//     }

//     #[test]
//     fn find_all_loops() {
//         let s = "....#.....
// .........#
// ..........
// ..#.......
// .......#..
// ..........
// .#..^.....
// ........#.
// #.........
// ......#...\n";
//         let m = Map::from_str(s);
//         let loops = m.find_all_loops();
//         println!("{:?}", loops);
//         assert_eq!(loops.len(), 6);
//     }

    #[test]
    fn looping_obstacles() {
        let s = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...\n";
        let m = Map::from_str(s);
        let obsts = m.looping_obstacles();
        assert_eq!(obsts.len(), 6);
    }
}