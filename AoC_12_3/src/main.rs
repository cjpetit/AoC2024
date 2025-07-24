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

fn part1(s: &str) -> i32 {
    let mut total = 0;
    let mut i: usize = 0;
    while i < s.len() {
        let c = s.as_bytes()[i] as char;
        i += 1;
        if c == 'm' {
            if let Ok(x) = scan_mul(s, &mut i) {
                total += x;
            }
        }
    }
    total
}

fn part2(s: &str) -> i32 {
    let mut total = 0;
    let mut i: usize = 0;
    let mut enable = true;
    while i < s.len() {
        let c = s.as_bytes()[i] as char;
        i += 1;
        match c {
            'm' => {
                if !enable {continue;}
                if let Ok(x) = scan_mul(s, &mut i) {
                    total += x;
                }
            },
            'd' => {
                if let Err(()) = scan_switch(s, &mut i, &mut enable) {}
            }
            _ => {}
        }
    }
    total
}

fn scan_mul(s: &str, i: &mut usize) -> Result<i32, ()> {
    assert_next(s, i, 'u')?;
    assert_next(s, i, 'l')?;
    assert_next(s, i, '(')?;
    let mut total = scan_num(s, i)?;
    assert_next(s, i, ',')?;
    total *= scan_num(s, i)?;
    assert_next(s, i, ')')?;
    Ok(total)
}

fn assert_next(s: &str, i: &mut usize, c: char) -> Result<(), ()> {
    if *i >= s.len() {return Err(());}
    if c == s.as_bytes()[*i] as char {
        *i += 1;
        return Ok(())
    }
    Err(()) 
}

fn scan_num(s: &str, i: &mut usize) -> Result<i32, ()> {
    let start = *i;
    loop {
        if *i >= s.len() {break;}
        let c = s.as_bytes()[*i] as char;
        if !c.is_numeric() {break;}
        *i += 1;
    }
    if *i != start {
        Ok(s[start..*i].parse().expect("parse error in scan_num"))
    } else {
        Err(())
    }
}

fn scan_switch(s: &str, i: &mut usize, enable: &mut bool) -> Result<(), ()> {
    assert_next(s, i, 'o')?;
    match assert_next(s, i, 'n') {
        Ok(()) => {
            assert_next(s, i, '\'')?;
            assert_next(s, i, 't')?;
            assert_next(s, i, '(')?;
            assert_next(s, i, ')')?;
            *enable = false;
        },
        Err(()) => {
            assert_next(s, i, '(')?;
            assert_next(s, i, ')')?;
            *enable = true;
        }
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn scan_num_ok() {
        let s = "sdfi2if";
        let mut i = 4;
        assert_eq!(scan_num(s, &mut i), Ok(2));
    }

    #[test]
    fn scan_num_err() {
        let s = "sdfi2if";
        let mut i = 3;
        assert_eq!(scan_num(s, &mut i), Err(()));
    }

    #[test]
    fn part1_() {
        let s = "$  mul(402,190))&<why(211,617)how()/;mul(506,313)[^^<!$#when(636,198),]mul(744,268)#&!wmul(2";
        assert_eq!(part1(s), 402*190+506*313+744*268);
    }

    #[test]
    fn part2_() {
        let s = "$  mul(402,190))&<why(211,617)how()/;mul(506,313)[^^<!$#when(636,198),]mul(744,268)#&!wmul(2";
        assert_eq!(part2(s), 402*190+506*313+744*268);
    }

    #[test]
    fn scan_disable() {
        let s = "$893hdiodon't()4%^mul(402,190))&<why(211,617)how()/;mul(506,313)[^^<!$#when(636,198),]mul(744,268)#&!wmul(2";
        assert_eq!(part2(s), 0);
    }

    #[test]
    fn scan_mixed() {
        let s = "$  mul(402,190))&<why(211,617)how()/;mul(506,313)[^^<!$#when(636,198),]mul(744,268)#&!what()&!;ul(206,770){/}don't()from()mul(260,967)-how() -/[^(mul(500,994)!:mul(391,833)#)>who(),where(376,378)why();$mul(394,346)%:]^from()>:mul(130,944)who()>where()select()}:mul(952,439)?/how()from()^$why()do()@*{mul(557,916)&(w";
        assert_eq!(part2(s), 402*190+506*313+744*268+557*916);
    }
}