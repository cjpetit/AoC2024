use core::str;
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

    println!("Part 1: {}", sum_of_middles(input.as_str()));
    println!("Part 2: {}", sum_of_corrected(input.as_str()));
}

fn sum_of_middles(s: &str) -> i32 {
    let (rules, mut i) = scan_rules(s);
    let reports = scan_reports(s, &mut i);
    let mut total = 0;
    let mut r = reports.iter();
    loop {
        if let Some(report) = r.next() {
            total += middle_if_valid(report, &rules);
        } else {break;}
    }
    total
}

fn sum_of_corrected(s: &str) -> i32 {
    let (rules, mut i) = scan_rules(s);
    let reports = scan_reports(s, &mut i);
    let mut total = 0;
    for report in reports.iter() {
        if !passes_all_rules(report, &rules) {
            let relevant = relevant_rules(report, &rules);
            let corrected = sequence_from_rules(&relevant, report);
            total += middle_of_report(&corrected);
        }
    }
    total
}

fn scan_rules(s: &str) -> (Vec<(i32, i32)>, usize) {
    let mut rules: Vec<(i32, i32)> = Vec::new();
    let mut i = 0;
    loop {
        if let Ok(x) = scan_num(s, &mut i) {
            if let Err(()) = assert_next(s, &mut i, '|') {continue;}
            if let Ok(y) = scan_num(s, &mut i) {
                rules.push((x, y));
                continue;
            }
        }
        i += 1;
        if i >= s.len() {panic!("EOF while scanning rules");}
        if s.as_bytes()[i-1..=i] == *"\n\n".as_bytes() || s.as_bytes()[i-3..=i] == *"\r\n\r\n".as_bytes() {
            return (rules, i+1);
        }
    }
}

fn scan_reports(s: &str, i: &mut usize) -> Vec<Vec<i32>> {
    let mut reports: Vec<Vec<i32>> = Vec::new();
    'outer: while *i < s.len() {
        let mut report: Vec<i32> = Vec::new();
        loop {
            if let Ok(x) = scan_num(s, i) {
                report.push(x);
                if let Err(()) = assert_next(s, i, ',') {
                    reports.push(report);
                    continue 'outer;
                }
            } else if *i >= s.len() {
                break 'outer;
            } else {
                *i += 1;
            }
        }
    }
    reports
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

fn assert_next(s: &str, i: &mut usize, c: char) -> Result<(), ()> {
    if *i >= s.len() {return Err(());}
    if c == s.as_bytes()[*i] as char {
        *i += 1;
        return Ok(())
    }
    Err(())
}

fn middle_if_valid(report: &Vec<i32>, rules: &Vec<(i32, i32)>) -> i32 {
    if passes_all_rules(report, rules) {
        middle_of_report(report)
    } else {0}
}

fn passes_all_rules(report: &Vec<i32>, rules: &Vec<(i32, i32)>) -> bool {
    for rule in rules.iter() {
        if !passes_rule(report, rule) {
            return false;
        }
    }
    true
}

fn passes_rule(report: &Vec<i32>, rule: &(i32, i32)) -> bool {
    let (x, y) = *rule;
    if let Some(i1) = report.iter().position(|&rule| rule == x) {
        if let Some(i2) = report.iter().position(|&rule| rule == y) {
            if i2 < i1 {return false;}
        }
    }
    true
}

fn middle_of_report(report: &Vec<i32>) -> i32 {
    if let Some(x) = report.get(report.len()/2) {
        *x
    } else {
        0
    }
}

fn relevant_rules(report: &Vec<i32>, rules: &Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    let mut relevant: Vec<(i32, i32)> = Vec::new();
    for rule in rules.iter() {
        let (x, y) = rule;
        if report.contains(x) && report.contains(y) {
            relevant.push(*rule);
        }
    }
    relevant
}

fn sequence_from_rules(rules: &Vec<(i32, i32)>, original: &Vec<i32>) -> Vec<i32> {
    let mut rules = rules.clone();
    let mut seq: Vec<i32> = Vec::new();
    while rules.len() > 0 {
        let prev_len = rules.len();
        if rules.len() == 1 {
            if original.contains(&rules[0].0) {seq.push(rules[0].0);}
            if original.contains(&rules[0].1) {seq.push(rules[0].1);}
            break;
        }
        let mut xs: Vec<i32> = Vec::new();
        let mut ys: Vec<i32> = Vec::new();
        for rule in rules.iter() {
            if !xs.contains(&rule.0) {
                xs.push(rule.0);
            }
            if !ys.contains(&rule.1) {
                ys.push(rule.1);
            }
        }
        for x in xs.iter() {
            if !ys.contains(x) {
                if original.contains(x) {seq.push(*x);}
                for rule in rules.clone().iter() {
                    if rule.0 == *x {
                        if rules.len() == 1 {
                            if original.contains(&rule.1) {seq.push(rule.1);}
                        }
                        let i = rules.iter().position(|r| r == rule).expect("rule removal indexing failed");
                        rules.remove(i);
                    }
                }
            }
        }
        if rules.len() == prev_len {panic!("sequence_from_rules infinite loop");}
    }
    seq
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn scan_rules_() {
        let s = "47|61\n75|61\n47|29\n75|13\n53|13\n\n75,47,61,53,29\n97,61,53,29,13\n";
        assert_eq!(scan_rules(s), (vec![(47, 61), (75, 61), (47, 29), (75, 13), (53, 13)], 31));
    }

    #[test]
    fn scan_reports_() {
        let s = "47|61\n75|61\n47|29\n75|13\n53|13\n\n75,47,61,53,29\n97,61,53,29,13\n";
        let mut i = 31;
        assert_eq!(scan_reports(s, &mut i), vec![vec![75,47,61,53,29], vec![97,61,53,29,13]]);
    }

    #[test]
    fn passes_rule_true() {
        let rule = (47, 53);
        let report = vec![75,47,61,53,29];
        assert!(passes_rule(&report, &rule));
    }

    #[test]
    fn passes_rule_false() {
        let rule = (47, 53);
        let report = vec![75,53,47,61,29];
        assert!(!passes_rule(&report, &rule));
    }

    #[test]
    fn middle_of_report_() {
        let report = vec![75,53,47,61,29];
        assert_eq!(middle_of_report(&report), 47);
    }

    #[test]
    fn sum_of_middles_() {
        let s = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
        assert_eq!(sum_of_middles(s), 143);
    }

    #[test]
    fn relevant_rules_() {
        let rules = vec![(47, 53), (26, 44), (27, 98), (4, 61), (75, 61), (23, 16), (42, 29)];
        let report = vec![75,53,47,61,29];
        assert_eq!(relevant_rules(&report, &rules), vec![(47, 53), (75, 61)]);
    }

    #[test]
    fn sequence_from_rules_() {
        let rules = vec![(47, 53), (21, 47), (21, 53), (47, 99), (53, 99)];
        let original = vec![47, 53, 21, 99];
        assert_eq!(sequence_from_rules(&rules, &original), vec![21, 47, 53, 99]);
    }

    #[test]
    fn sum_of_corrected_() {
        let s = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
        assert_eq!(sum_of_corrected(s), 123);
    }
}