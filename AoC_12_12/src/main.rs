use std::{env, fs::File, io::Read, path::Path, collections::{HashSet, HashMap}};

#[derive(PartialEq, Debug, Clone)]
struct Region {
    letter: char,
    coords: Vec<(usize, usize)>
}

impl Region {
    fn from(letter: char, coords: Vec<(usize, usize)>) -> Self {
        Self {
            letter,
            coords
        }
    }

    fn price1(&self) -> usize {
        self.area() * self.perimeter()
    }

    fn price2(&self) -> i32 {
        self.area() as i32 * self.sides()
    }

    fn area(&self) -> usize {
        self.coords.len()
    }

    fn perimeter(&self) -> usize {
        let mut perimeter = self.area() * 4;
        for (i, coord1) in self.coords.iter().enumerate() {
            for coord2 in self.coords[i..].iter() {
                if coord1.0 == coord2.0 && (coord1.1 == coord2.1 + 1 || coord1.1 + 1 == coord2.1) {
                    perimeter -= 2;
                }
                if coord1.1 == coord2.1 && (coord1.0 == coord2.0 + 1 || coord1.0 + 1 == coord2.0) {
                    perimeter -= 2;
                }
            }
        }
        perimeter
    }

    fn sides(&self) -> i32 {
        let mut sides: i32 = 0;
        let mut visited: Vec<(usize, usize)> = vec![];
        for index in self.coords.iter() {
            let cardinals = Self::cardinals(*index, &visited);
            match cardinals.len() {
                0 => {sides += 4;},
                4 => {sides -= 4;},
                1 => {
                    sides += 2 * Self::adjacent_corners(*index, cardinals[0], &visited) as i32;
                },
                2 => {
                    if Self::opposite(cardinals[0], cardinals[1]) {
                        match Self::corners(*index, &visited) {
                            0 => {sides -= 2;},
                            x => {sides += (x as i32 - 2) * 2;}
                        }
                    } else {
                        sides += (Self::side_corners(*index, &cardinals, &visited) as i32 - 1) * 2;
                    }
                },
                3 => {sides -= (2 - Self::outside_corners(*index, &cardinals, &visited) as i32) * 2;},
                _ => {panic!("sides call: >4 cardinals");}
            }
            visited.push(*index);
        }
        sides
    }

    fn cardinals(index: (usize, usize), visited: &Vec<(usize, usize)>) -> Vec<(usize, usize)> {
        let mut adjacent = vec![(index.0 + 1, index.1), (index.0, index.1 + 1)];
        if index.0 > 0 {adjacent.push((index.0 - 1, index.1));}
        if index.1 > 0 {adjacent.push((index.0, index.1 - 1));}
        intersection(visited, &adjacent)
    }

    fn corners(index: (usize, usize), visited: &Vec<(usize, usize)>) -> u8 {
        let mut corners = visited.contains(&(index.0 + 1, index.1 + 1)) as u8;
        if index.0 > 0 {corners += visited.contains(&(index.0 - 1, index.1 + 1)) as u8;}
        if index.1 > 0 {corners += visited.contains(&(index.0 + 1, index.1 - 1)) as u8;}
        if index.0 > 0 && index.1 > 0 {corners += visited.contains(&(index.0 - 1, index.1 - 1)) as u8;}
        corners
    }

    fn adjacent_corners(index: (usize, usize), cardinal: (usize, usize), visited: &Vec<(usize, usize)>) -> u8 {
        if cardinal.0 == index.0 {
            if index.0 > 0 {visited.contains(&(cardinal.0 - 1, cardinal.1)) as u8 + visited.contains(&(cardinal.0 + 1, cardinal.1)) as u8}
            else {visited.contains(&(cardinal.0 + 1, cardinal.1)) as u8}
        } else if cardinal.1 == index.1 {
            if cardinal.1 > 0 {visited.contains(&(cardinal.0, cardinal.1 - 1)) as u8 + visited.contains(&(cardinal.0, cardinal.1 + 1)) as u8}
            else {visited.contains(&(cardinal.0, cardinal.1 + 1)) as u8}
        } else {panic!("adjacent_corners call: index and cardinal missing common coordinate");}
    }

    fn side_corners(index: (usize, usize), cardinals: &Vec<(usize, usize)>, visited: &Vec<(usize, usize)>) -> u8 {
        if cardinals.contains(&(index.0 + 1, index.1)) && cardinals.contains(&(index.0, index.1 + 1)) {
            (index.0 > 0 && visited.contains(&(index.0 - 1, index.1 + 1))) as u8 + (index.1 > 0 && visited.contains(&(index.0 + 1, index.1 - 1))) as u8
        } else if index.0 > 0 && cardinals.contains(&(index.0 - 1, index.1)) && cardinals.contains(&(index.0, index.1 + 1)) {
            (index.0 > 0 && index.1 > 0 && visited.contains(&(index.0 - 1, index.1 - 1))) as u8 + (visited.contains(&(index.0 + 1, index.1 + 1))) as u8
        } else if index.1 > 0 && cardinals.contains(&(index.0, index.1 - 1)) && cardinals.contains(&(index.0 + 1, index.1)) {
            (index.0 > 0 && index.1 > 0 && visited.contains(&(index.0 - 1, index.1 - 1))) as u8 + (visited.contains(&(index.0 + 1, index.1 + 1))) as u8
        } else if index.0 > 0 && index.1 > 0 && cardinals.contains(&(index.0 - 1, index.1)) && cardinals.contains(&(index.0, index.1 - 1)) {
            (index.0 > 0 && visited.contains(&(index.0 - 1, index.1 + 1))) as u8 + (index.1 > 0 && visited.contains(&(index.0 + 1, index.1 - 1))) as u8
        } else {panic!("side_corners call: invalid cardinals")}
    }

    fn outside_corners(index: (usize, usize), cardinals: &Vec<(usize, usize)>, visited: &Vec<(usize, usize)>) -> u8 {
        if !cardinals.contains(&(index.0 + 1, index.1)) {
            (index.1 > 0 && visited.contains(&(index.0 + 1, index.1 - 1))) as u8 + visited.contains(&(index.0 + 1, index.1 + 1)) as u8
        } else if !cardinals.contains(&(index.0, index.1 + 1)) {
            (index.0 > 0 && visited.contains(&(index.0 - 1, index.1 + 1))) as u8 + visited.contains(&(index.0 + 1, index.1 + 1)) as u8
        } else if index.0 == 0 || !cardinals.contains(&(index.0 - 1, index.1)) {
            (index.0 > 0 && visited.contains(&(index.0 - 1, index.1 + 1))) as u8 + (index.0 > 0 && index.1 > 0 && visited.contains(&(index.0 - 1, index.1 - 1))) as u8
        } else if index.1 == 0 || !cardinals.contains(&(index.0, index.1 - 1)) {
            (index.1 > 0 && visited.contains(&(index.0 + 1, index.1 - 1))) as u8 + (index.0 > 0 && index.1 > 0 && visited.contains(&(index.0 - 1, index.1 - 1))) as u8
        } else {panic!("outside_corners call: invalid cardinals");}
    }

    fn opposite(index1: (usize, usize), index2: (usize, usize)) -> bool {
        if index1.0 == index2.0 && (index1.1 + 2 == index2.1 || index1.1 == index2.1 + 2) {return true;}
        index1.1 == index2.1 && (index1.0 + 2 == index2.0 || index1.0 == index2.0 + 2)
    }

    #[allow(dead_code)]
    fn to_hashset(&self) -> HashSet<(usize, usize)> {
        let x = self.coords.clone();
        HashSet::from_iter(x)
    }

    #[allow(dead_code)]
    fn report(&self) {
        let area = self.area();
        let perimeter = self.perimeter();
        let sides = self.sides();
        assert!(perimeter as i32 >= sides);
        match (area, area < 1) {
            (_,true) => {panic!("area < 1");},
            (1,_) => {
                assert_eq!(perimeter, 4);
                assert_eq!(sides, 4);
            },
            (2,_) => {
                assert_eq!(perimeter, 6);
                assert_eq!(sides, 4);
            },
            _ => {println!("'{}': area={}, perimeter={}, sides={}", self.letter, area, perimeter, sides);}
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
    let mut price = 0;
    let map = read_to_map(s);
    let regions = get_all_regions(&map);
    for region in regions.iter() {
        price += region.price1();
    }
    price
}

fn part2(s: &str) -> i32 {
    let mut price = 0;
    let map = read_to_map(s);
    let regions = get_all_regions(&map);
    for region in regions.iter() {
        price += region.price2();
    }
    price
}

fn read_to_map(s: &str) -> HashMap<char, Vec<(usize, usize)>> {
    let mut letter_coords: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    for (i, row) in s.trim().split("\n").enumerate() {
        for (j, byte) in row.trim().as_bytes().iter().enumerate() {
            let c = *byte as char;
            if let Some(c_ref) = letter_coords.get_mut(&c) {
                c_ref.push((i, j));
            } else {
                letter_coords.insert(c, Vec::from([(i, j)]));
            }
        }
    }
    letter_coords
}

fn get_all_regions(map: &HashMap<char, Vec<(usize, usize)>>) -> Vec<Region> {
    let mut regions: Vec<Region> = vec![];
    for (letter, set) in map.iter() {
        let mut letter_regions = get_letter_regions(*letter, set);
        regions.append(&mut letter_regions);
    }
    regions
}

fn get_letter_regions(letter: char, coords: &Vec<(usize, usize)>) -> Vec<Region> {
    let mut regions: Vec<Region> = vec![];
    'index: for index in coords.iter() {
        for region in regions.iter() {
            if region.coords.contains(&index) {continue 'index;}
        }
        regions.push(get_region(letter, coords, *index));
    }
    regions
}

fn get_region(letter: char, coords: &Vec<(usize, usize)>, index: (usize, usize)) -> Region {
    let mut reg_coords = Vec::from([index]);
    let mut expanse = get_expanse(&reg_coords, &reg_coords);
    let mut int = intersection(&expanse, &coords);
    while !int.is_empty() {
        for index in int.iter() {
            reg_coords.push(*index);
        }
        expanse = get_expanse(&reg_coords, &int);
        int = intersection(&expanse, &coords);
    }
    Region::from(letter, reg_coords)
}

fn get_expanse(reg_coords: &Vec<(usize, usize)>, indices: &Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    let mut expanse = Vec::new();
    for index in indices.iter() {
        if !reg_coords.contains(&(index.0 + 1, index.1)) && !expanse.contains(&(index.0 + 1, index.1)) {expanse.push((index.0 + 1, index.1));}
        if !reg_coords.contains(&(index.0, index.1 + 1)) && !expanse.contains(&(index.0, index.1 + 1)) {expanse.push((index.0, index.1 + 1));}
        if index.0 > 0 && !reg_coords.contains(&(index.0 - 1, index.1)) && !expanse.contains(&(index.0 - 1, index.1)) {expanse.push((index.0 - 1, index.1));}
        if index.1 > 0 && !reg_coords.contains(&(index.0, index.1 - 1)) && !expanse.contains(&(index.0, index.1 - 1)) {expanse.push((index.0, index.1 - 1));}
    }
    expanse
}

fn intersection(v1: &Vec<(usize, usize)>, v2: &Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    let mut int: Vec<(usize, usize)> = vec![];
    for elem in v1.iter() {
        if v2.contains(elem) {int.push(*elem);}
    }
    int
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn read_to_map_() {
        let map = read_to_map("AABC\nAABD\nEEEA\nEEEA\n");
        assert_eq!(*map.get(&'A').unwrap(), Vec::from([(0, 0), (0, 1), (1, 0), (1, 1), (2, 3), (3, 3)]));
        assert_eq!(*map.get(&'B').unwrap(), Vec::from([(0, 2), (1, 2)]));
        assert_eq!(*map.get(&'C').unwrap(), Vec::from([(0, 3)]));
        assert_eq!(*map.get(&'D').unwrap(), Vec::from([(1, 3)]));
        assert_eq!(*map.get(&'E').unwrap(), Vec::from([(2, 0), (2, 1), (2, 2), (3, 0), (3, 1), (3, 2)]));
        assert_eq!(map.get(&'F'), None);
    }

    #[test]
    fn get_region_() {
        let map = read_to_map("AABC\nAABD\nEEEA\nEEEA\n");
        let set = map.get(&'A').unwrap();
        let r = get_region('A', set, (0, 0));
        assert_eq!(HashSet::from_iter(r.coords), HashSet::from([(0, 0), (0, 1), (1, 0), (1, 1)]));
        let set = map.get(&'E').unwrap();
        let r = get_region('E', set, (3, 1));
        assert_eq!(HashSet::from_iter(r.coords), HashSet::from([(2, 0), (2, 1), (2, 2), (3, 0), (3, 1), (3, 2)]));
    }

    #[test]
    fn get_letter_regions_() {
        let map = read_to_map("AABC\nAABD\nEEEA\nEEEA\n");
        let set = map.get(&'A').unwrap();
        let regions = get_letter_regions('A', set);
        assert_eq!(regions.len(), 2);
        let expected = [HashSet::from([(0usize, 0usize), (0, 1), (1, 0), (1, 1)]), HashSet::from([(2, 3), (3, 3)])];
        for r in regions.iter() {
            assert!(expected.contains(&r.to_hashset()))
        }
    }

    #[test]
    fn get_all_regions_() {
        let map = read_to_map("AABC\nAABD\nEEEA\nEEEA\n");
        let regions = get_all_regions(&map);
        assert_eq!(regions.len(), 6);
    }

    #[test]
    fn perimeter() {
        let r = Region::from('A', Vec::from([(1, 1), (2, 1), (2, 2), (3, 1), (3, 2)]));
        assert_eq!(r.perimeter(), 10);
    }

    #[test]
    fn price1() {
        let r = Region::from('A', Vec::from([(1, 1), (2, 1), (2, 2), (3, 1), (3, 2)]));
        assert_eq!(r.price1(), 50);
    }

    #[test]
    fn part1_() {
        let price = part1("RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
");
        assert_eq!(price, 1930);
    }

    #[test]
    fn sides_() {
        let map = read_to_map("
AAEEEA
AEEAEE
AEEAAA
AAEEEE
AEEAAA
AAAAAA
");
        let e_regions = get_letter_regions('E', map.get(&'E').unwrap());
        assert_eq!(e_regions.len(), 1);
        println!("{:?}", e_regions[0]);
        assert_eq!(e_regions[0].sides(), 20);
    }

    #[test]
    fn part2_1() {
        let price = part2("RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
");
        assert_eq!(price, 1206);
    }

    #[test]
    fn part2_2() {
        let price = part2("EEEEE
EXXXX
EEEEE
EXXXX
EEEEE
");
        assert_eq!(price, 236);
    }

    #[test]
    fn part2_3() {
        let price = part2("AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA
");
        assert_eq!(price, 368);
    }
}