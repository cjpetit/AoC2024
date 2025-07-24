use std::{env, fs::File, io::Read, path::Path};

#[derive(PartialEq, Clone, Debug)]
struct Computer {
    reg_a: u32,
    reg_b: u32,
    reg_c: u32,
    program: Vec<u8>,
    pointer: usize,
    output: Vec<u32>
}

impl Computer {
    fn new(reg_a: u32, reg_b: u32, reg_c: u32, program: Vec<u8>) -> Self {
        Self {
            reg_a, reg_b, reg_c,
            program,
            pointer: 0,
            output: vec![]
        }
    }

    #[allow(dead_code)]
    fn compose(reg_a: u32, reg_b: u32, reg_c: u32, program: Vec<u8>, pointer: usize, output: Vec<u32>) -> Self {
        Self {
            reg_a, reg_b, reg_c, program, pointer, output
        }
    }

    fn execute_all(&mut self) {
        while self.execute() {}
    }

    fn execute(&mut self) -> bool {
        match (self.program.get(self.pointer), self.program.get(self.pointer + 1)) {
            (None,_) => false,
            (_,None) => {panic!("program ends on lone opcode");},
            (Some(0), Some(x)) => {
                self.adv(*x);
                true
            },
            (Some(1), Some(x)) => {
                self.bxl(*x);
                true
            },
            (Some(2), Some(x)) => {
                self.bst(*x);
                true
            },
            (Some(3), Some(x)) => {
                self.jnz(*x);
                true
            },
            (Some(4), Some(_)) => {
                self.bxc();
                true
            },
            (Some(5), Some(x)) => {
                self.out(*x);
                true
            },
            (Some(6), Some(x)) => {
                self.bdv(*x);
                true
            },
            (Some(7), Some(x)) => {
                self.cdv(*x);
                true
            },
            (Some(n),_) => {panic!("expected opcode 0 through 7, found {}", n);}
        }
    }

    fn output_str(&self) -> String {
        let mut output_iter = self.output.iter();
        let mut op_str = match output_iter.next() {
            None => {return "".to_string();},
            Some(n) => n.to_string()
        };
        for n in output_iter {
            op_str.extend([',']);
            op_str.extend(n.to_string().chars());
        }
        op_str
    }

    fn adv(&mut self, operand: u8) {
        self.reg_a /= 2_u32.pow(self.combo(operand));
        self.pointer += 2;
    }

    // fn r_adv(&self, operand: u8) {}

    fn bxl(&mut self, operand: u8) {
        assert!(operand < 8);
        let mut b_bin = to_binary(self.reg_b);
        let mut op_bin = to_binary(operand as u32);
        let mut xor = vec![];
        loop {
            match (b_bin.pop(), op_bin.pop()) {
                (None, None) => {break;},
                (Some(b), None) => {xor.push(b);},
                (None, Some(b)) => {xor.push(b);},
                (Some(false), Some(b)) => {xor.push(b);},
                (Some(true), Some(b)) => {xor.push(!b);}
            }
        }
        xor = xor.iter().rev().map(|&b| b).collect();
        self.reg_b = to_decimal(xor);
        self.pointer += 2;
    }

    fn bst(&mut self, operand: u8) {
        self.reg_b = self.combo(operand) % 8;
        self.pointer += 2;
    }

    fn jnz(&mut self, operand: u8) {
        assert!(operand < 8);
        if self.reg_a == 0 {self.pointer += 2;}
        else {self.pointer = operand as usize;}
    }

    fn bxc(&mut self) {
        let mut b_bin = to_binary(self.reg_b);
        let mut c_bin = to_binary(self.reg_c);
        let mut xor = vec![];
        loop {
            match (b_bin.pop(), c_bin.pop()) {
                (None, None) => {break;},
                (Some(b), None) => {xor.push(b);},
                (None, Some(b)) => {xor.push(b);},
                (Some(false), Some(b)) => {xor.push(b);},
                (Some(true), Some(b)) => {xor.push(!b);}
            }
        }
        xor = xor.iter().rev().map(|&b| b).collect();
        self.reg_b = to_decimal(xor);
        self.pointer += 2;
    }

    fn out(&mut self, operand: u8) {
        self.output.push(self.combo(operand) % 8);
        self.pointer += 2;
    }

    fn bdv(&mut self, operand: u8) {
        self.reg_b = self.reg_a / 2_u32.pow(self.combo(operand));
        self.pointer += 2;
    }

    fn cdv(&mut self, operand: u8) {
        self.reg_c = self.reg_a / 2_u32.pow(self.combo(operand));
        self.pointer += 2;
    }

    fn combo(&self, operand: u8) -> u32 {
        match operand {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,
            4 => self.reg_a,
            5 => self.reg_b,
            6 => self.reg_c,
            7 => {panic!("reserved combo operand 7");},
            n => {panic!("expected operand 0 through 7, found {}", n);}
        }
    }
}

// #[derive(PartialEq, Clone, Debug)]
// struct Corrupted {
//     reg_a: Ambig,
//     reg_b: Ambig,
//     reg_c: Ambig,
//     program: Vec<u8>,
//     pointer: usize,
//     output: Vec<u32>
// }

// #[derive(PartialEq, Clone, Debug)]
// enum Ambig {
//     MinMax(u32, u32),
//     Mod8(u8)
// }

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

fn part1(s: &str) -> String {
    let mut comp = read_computer(s);
    comp.execute_all();
    comp.output_str()
} 

fn part2(s: &str) -> u32 {
    let base = read_computer(s);
    for n in 33750000..u32::MAX {
        println!("testing {}", n);
        let mut variant = base.clone();
        variant.reg_a = n;
        while variant.execute() {
            if let Err(()) = compare_output_program(&variant) {continue;}
        }
        if compare_output_program(&variant) == Ok(true) {return n;}
    }
    0
}

fn compare_output_program(comp: &Computer) -> Result<bool, ()> {
    let mut output = comp.output.iter().rev();
    let mut program = comp.program.iter().rev();
    loop {
        match (output.next(), program.next()) {
            (Some(_), None) => {return Err(());},
            (Some(x), Some(y)) => {
                if *x != *y as u32 {return Err(());}
            },
            (None, Some(_)) => {return Ok(false);},
            (None, None) => {return Ok(true);}
        }
    }
}

fn read_computer(s: &str) -> Computer {
    let mut line_iter = s.trim().split("\n").map(|l| l.trim());
    let a_phrase = line_iter.next().unwrap();
    let b_phrase = line_iter.next().unwrap();
    let c_phrase = line_iter.next().unwrap();
    assert_eq!(line_iter.next().unwrap(), "");
    let prog_phrase = line_iter.next().unwrap();
    assert!(line_iter.next().is_none());

    assert_eq!(&a_phrase[0..12], "Register A: ");
    let a = a_phrase[12..].parse::<u32>().expect("parse error");
    assert_eq!(&b_phrase[0..12], "Register B: ");
    let b = b_phrase[12..].parse::<u32>().expect("parse error");
    assert_eq!(&c_phrase[0..12], "Register C: ");
    let c = b_phrase[12..].parse::<u32>().expect("parse error");
    assert_eq!(&prog_phrase[0..9], "Program: ");

    let mut prog = Vec::<u8>::new();
    for num in prog_phrase[9..].split(",") {
        prog.push(num.parse().expect("parse error"));
    }

    Computer::new(a, b, c, prog)
}

// fn step_back(v: Vec<Computer>) -> Vec<Computer> {
//     let mut v = v;
//     let mut new_v = vec![];

//     new_v
// }

fn to_binary(x: u32) -> Vec<bool> {
    let mut x = x;
    let mut v = vec![];
    let mut divisor = 1;
    while divisor * 2 <= x {divisor *= 2;}
    while divisor > 0 {
        if x / divisor == 1 {
            v.push(true);
            x %= divisor;
        } else {
            assert!(x < divisor);
            v.push(false);
        }
        divisor /= 2;
    }
    v
}

fn to_decimal(v: Vec<bool>) -> u32 {
    let mut v = v;
    let mut digit = 1;
    let mut x = 0;
    loop {
        match v.pop() {
            None => {return x;},
            Some(true) => {x += digit;},
            Some(false) => {}
        }
        digit *= 2;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn read_computer_() {
        let comp = read_computer("Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0"
);
        assert_eq!(comp, Computer::compose(729, 0, 0, vec![0, 1, 5, 4, 3, 0], 0, vec![]));
    }

    #[test]
    fn adv() {
        let mut comp = Computer::new(100, 3, 1, vec![0, 5]);
        comp.adv(5);
        assert_eq!(comp, Computer::compose(12, 3, 1, vec![0, 5], 2, vec![]));
    }

    #[test]
    fn execute_adv() {
        let mut comp = Computer::new(100, 3, 1, vec![0, 5]);
        comp.execute();
        assert_eq!(comp, Computer::compose(12, 3, 1, vec![0, 5], 2, vec![]));
    }

    #[test]
    fn bxl() {
        let mut comp = Computer::new(100, 21, 1, vec![1, 7]);
        comp.bxl(7);
        assert_eq!(comp, Computer::compose(100, 18, 1, vec![1, 7], 2, vec![]));
    }

    #[test]
    fn execute_bxl() {
        let mut comp = Computer::new(100, 21, 1, vec![1, 7]);
        comp.execute();
        assert_eq!(comp, Computer::compose(100, 18, 1, vec![1, 7], 2, vec![]));
    }

    #[test]
    fn bst() {
        let mut comp = Computer::new(100, 21, 1, vec![2, 6]);
        comp.bst(6);
        assert_eq!(comp, Computer::compose(100, 1, 1, vec![2, 6], 2, vec![]));
    }

    #[test]
    fn execute_bst() {
        let mut comp = Computer::new(100, 21, 1, vec![2, 6]);
        comp.execute();
        assert_eq!(comp, Computer::compose(100, 1, 1, vec![2, 6], 2, vec![]));
    }

    #[test]
    fn jnz_a0() {
        let mut comp = Computer::new(0, 21, 1, vec![3, 4]);
        comp.jnz(4);
        assert_eq!(comp, Computer::compose(0, 21, 1, vec![3, 4], 2, vec![]));
    }

    #[test]
    fn jnz_jump() {
        let mut comp = Computer::new(10, 21, 1, vec![3, 4]);
        comp.jnz(4);
        assert_eq!(comp, Computer::compose(10, 21, 1, vec![3, 4], 4, vec![]));
    }

    #[test]
    fn execute_jnz() {
        let mut comp = Computer::new(10, 21, 1, vec![3, 4]);
        comp.execute();
        assert_eq!(comp, Computer::compose(10, 21, 1, vec![3, 4], 4, vec![]));
    }

    #[test]
    fn bxc() {
        let mut comp = Computer::new(10, 100, 55, vec![4, 4]);
        comp.bxc();
        assert_eq!(comp, Computer::compose(10, 83, 55, vec![4, 4], 2, vec![]));
    }

    #[test]
    fn execute_bxc() {
        let mut comp = Computer::new(10, 100, 55, vec![4, 4]);
        comp.execute();
        assert_eq!(comp, Computer::compose(10, 83, 55, vec![4, 4], 2, vec![]));
    }

    #[test]
    fn out() {
        let mut comp = Computer::new(10, 100, 55, vec![5, 5]);
        comp.out(5);
        assert_eq!(comp, Computer::compose(10, 100, 55, vec![5, 5], 2, vec![4]));
    }

    #[test]
    fn execute_out() {
        let mut comp = Computer::new(10, 100, 55, vec![5, 5]);
        comp.execute();
        assert_eq!(comp, Computer::compose(10, 100, 55, vec![5, 5], 2, vec![4]));
    }

    #[test]
    fn bdv() {
        let mut comp = Computer::new(10, 100, 55, vec![6, 2]);
        comp.bdv(2);
        assert_eq!(comp, Computer::compose(10, 2, 55, vec![6, 2], 2, vec![]));
    }

    #[test]
    fn execute_bdv() {
        let mut comp = Computer::new(10, 100, 55, vec![6, 2]);
        comp.execute();
        assert_eq!(comp, Computer::compose(10, 2, 55, vec![6, 2], 2, vec![]));
    }

    #[test]
    fn cdv() {
        let mut comp = Computer::new(123, 100, 3, vec![7, 6]);
        comp.cdv(6);
        assert_eq!(comp, Computer::compose(123, 100, 15, vec![7, 6], 2, vec![]));
    }

    #[test]
    fn execute_cdv() {
        let mut comp = Computer::new(123, 100, 3, vec![7, 6]);
        comp.execute();
        assert_eq!(comp, Computer::compose(123, 100, 15, vec![7, 6], 2, vec![]));
    }

    #[test]
    fn execute_all() {
        let mut comp = Computer::compose(729, 0, 0, vec![0, 1, 5, 4, 3, 0], 0, vec![]);
        comp.execute_all();
        assert_eq!(comp, Computer::compose(0, 0, 0, vec![0, 1, 5, 4, 3, 0], 6, vec![4, 6, 3, 5, 6, 3, 5, 2, 1, 0]));
    }

    #[test]
    fn output_str() {
        let comp = Computer::compose(0, 0, 0, vec![0, 1, 5, 4, 3, 0], 6, vec![4, 6, 3, 5, 6, 3, 5, 2, 1, 0]);
        assert_eq!(comp.output_str(), "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn to_binary_17() {
        assert_eq!(to_binary(17), vec![true, false, false, false, true]);
    }

    #[test]
    fn to_binary_126() {
        assert_eq!(to_binary(126), vec![true, true, true, true, true, true, false]);
    }

    #[test]
    fn to_decimal_26() {
        assert_eq!(to_decimal(vec![true, true, false, true, false]), 26);
    }

    #[test]
    fn to_decimal_67() {
        assert_eq!(to_decimal(vec![true, false, false, false, false, true, true]), 67);
    }
}
