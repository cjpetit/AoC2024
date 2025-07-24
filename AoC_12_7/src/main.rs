use std::{env, fs::File, io::Read, path::Path};

#[derive(Debug, PartialEq, Clone)]
struct Equation {
    value: i64,
    operands: Vec<i64>
}

// impl Equation {
//     fn solve(&mut self) -> Result<(), ()> {

//     }
// }

#[derive(Debug, PartialEq, Copy, Clone)]
enum Operator {
    Mul, Add, Conc
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

fn part1(s: &str) -> i64 {
    let mut calibration = 0;
    let equations = scan_equations(s).expect("equation scan error");
    for eq in equations.iter() {
        if let Ok(ops) = match_oper(eq.value, &eq.operands) {
            if !ops.contains(&Operator::Conc) {
                calibration += eq.value as i64;
            }
        }
    }
    calibration
}

fn part2(s: &str) -> i64 {
    let mut calibration = 0;
    let equations = scan_equations(s).expect("equation scan error");
    for eq in equations.iter() {
        if let Ok(_) = match_oper(eq.value, &eq.operands) {
            calibration += eq.value as i64;
        }
    }
    calibration
}

fn scan_equations(s: &str) -> Result<Vec<Equation>, ()> {
    let mut equations: Vec<Equation> = Vec::new();
    let mut i = 0;
    'outer: while i < s.len() {
        let value = scan_num(s, &mut i)? as i64;
        let mut operands: Vec<i64> = Vec::new();
        assert_next(s, &mut i, ':')?;
        'inner: loop {
            if let Ok(()) = assert_next(s, &mut i, ' ') {
                if let Ok(num) = scan_num(s, &mut i) {
                    operands.push(num);
                    continue 'inner;
                }
            }
            if i >= s.len() {
                equations.push(Equation { value, operands });
                break 'outer;
            }
            let _ = assert_next(s, &mut i, '\r');
            assert_next(s, &mut i, '\n')?;
            break 'inner;
        }
        equations.push(Equation { value, operands });
    }
    Ok(equations)
}

fn match_oper(value: i64, operands: &Vec<i64>) -> Result<Vec<Operator>, ()> {
    if operands.len() < 2 {
        panic!("match_oper call on vec len <2");
    } else if operands.len() == 2 {
        if operands[0] + operands[1] == value {
            return Ok(vec![Operator::Add]);
        } else if operands[0] * operands[1] == value {
            return Ok(vec![Operator::Mul]);
        } else {
            let mut r = operands[1];
            let mut mul_factor = 10;
            loop {
                r /= 10;
                if r == 0 {break;}
                else {mul_factor *= 10;}
            }
            if operands[0] * mul_factor + operands[1] == value {
                return Ok(vec![Operator::Conc]);
            }
            return Err(());
        }
    } else {
        let mut ops_mul = operands.clone();
        ops_mul[1] *= ops_mul[0];
        ops_mul.remove(0);
        if let Ok(v) = match_oper(value, &ops_mul) {
            let mut all_ops = vec![Operator::Mul];
            all_ops.extend(v.iter());
            return Ok(all_ops);
        }
        let mut ops_add = operands.clone();
        ops_add[1] += ops_add[0];
        ops_add.remove(0);
        if let Ok(v) = match_oper(value, &ops_add) {
            let mut all_ops = vec![Operator::Add];
            all_ops.extend(v.iter());
            return Ok(all_ops);
        }
        let mut ops_conc = operands.clone();
        let mut r = ops_conc[1];
        let mut mul_factor = 10;
        loop {
            r /= 10;
            if r == 0 {break;}
            else {mul_factor *= 10;}
        }
        ops_conc[1] += ops_conc[0] * mul_factor;
        ops_conc.remove(0);
        if let Ok(v) = match_oper(value, &ops_conc) {
            let mut all_ops = vec![Operator::Conc];
            all_ops.extend(v.iter());
            return Ok(all_ops);
        }
        return Err(());
    }
}

fn scan_num(s: &str, i: &mut usize) -> Result<i64, ()> {
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

fn assert_next(s: &str, i: &mut usize, c: char) -> Result<(), ()> {
    if *i >= s.len() {return Err(());}
    if c == s.as_bytes()[*i] as char {
        *i += 1;
        return Ok(())
    }
    Err(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn scan_equations_() {
        let s = "3: 1 1 1
10: 5 2";
        assert_eq!(scan_equations(s).unwrap(), vec![Equation { value: 3, operands: vec![1, 1, 1], }, Equation { value: 10, operands: vec![5, 2], }]);
    }

    #[test]
    fn match_oper_2() {
        use Operator::*;
        let e = Equation { value: 6, operands: vec![2, 3], };
        assert_eq!(match_oper(e.value, &e.operands).unwrap(), vec![Mul]);
    }

    #[test]
    fn match_oper_3() {
        use Operator::*;
        let e = Equation { value: 15, operands: vec![3, 2, 3], };
        assert_eq!(match_oper(e.value, &e.operands).unwrap(), vec![Add, Mul]);
    }

    #[test]
    fn match_oper_4() {
        let e = Equation { value: 15, operands: vec![3, 2, 3, 9], };
        assert_eq!(match_oper(e.value, &e.operands), Err(()));
    }
}