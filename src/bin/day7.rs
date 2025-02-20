use clap::Parser;
use itertools::Itertools;
use std::fmt;
use std::fs;
use std::path::PathBuf;

#[derive(Parser, Debug)]
struct Cli {
    in_path: PathBuf,
}

#[derive(Clone, Debug)]
struct Equation {
    result: i64,
    numbers: Vec<i64>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Operator {
    Add,
    Mul,
    Cat,
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let out = match *self {
            Operator::Add => "+",
            Operator::Mul => "*",
            Operator::Cat => "||",
        };
        write!(f, "{out}")
    }
}

const ADD_MUL: [Operator; 2] = [Operator::Add, Operator::Mul];
const ALL_OPS: [Operator; 3] = [Operator::Add, Operator::Mul, Operator::Cat];

#[derive(Debug)]
struct Calculator {
    accumulator: i64,
}

impl Calculator {
    fn new(num: i64) -> Self {
        Self { accumulator: num }
    }

    fn apply(&mut self, op: Operator, num: i64) {
        fn cat(left: i64, right: i64) -> i64 {
            let mut multiplicator = 10;
            while multiplicator < right {
                multiplicator *= 10;
            }
            left * multiplicator + right
        }
        let new = match op {
            Operator::Add => self.accumulator + num,
            Operator::Mul => self.accumulator * num,
            Operator::Cat => cat(self.accumulator, num),
        };
        self.accumulator = new;
    }
}

#[allow(dead_code)]
fn print_eq(eq: &Equation, ops: &[&Operator]) {
    let mut out = format!("{} = ", eq.result);
    for x in eq
        .numbers
        .iter()
        .map(|n| n.to_string())
        .interleave(ops.iter().map(|o| o.to_string()))
    {
        out += " ";
        out += &x;
    }
    println!("{}", out);
}

fn main() {
    let cli = Cli::parse();
    let f = fs::read_to_string(cli.in_path).unwrap();
    let i = parse_input(&f);
    let res = part1(i.clone());
    println!("{res}");
    let res = part2(i);
    println!("{res}");
}

fn parse_input(input: &str) -> Vec<Equation> {
    let mut res = Vec::new();
    for l in input.lines() {
        let mut splitted = l.split(':');
        let result = splitted.next().unwrap().parse().unwrap();
        let numbers = splitted
            .next()
            .unwrap()
            .trim()
            .split(' ')
            .map(|n| n.parse().unwrap())
            .collect();
        res.push(Equation { result, numbers });
    }
    res
}

fn check_eq(eq: Equation, allowed_ops: &[Operator]) -> bool {
    let num_ops = eq.numbers.len() - 1;
    for ops in (0..num_ops).map(|_| allowed_ops).multi_cartesian_product() {
        //print!("Checking ");
        //print_eq(&eq, &ops);
        let mut nums = eq.numbers.iter();
        let first = nums.next().unwrap();
        let mut cal = Calculator::new(*first);
        for o in ops.iter() {
            let right = nums.next().unwrap();
            cal.apply(**o, *right)
        }
        if cal.accumulator == eq.result {
            return true;
        }
    }
    false
}

fn part1(input: Vec<Equation>) -> i64 {
    let mut res = 0;
    for eq in input {
        let eq_res = eq.result;
        if check_eq(eq, &ADD_MUL) {
            res += eq_res;
        }
    }
    res
}

fn part2(input: Vec<Equation>) -> i64 {
    let mut res = 0;
    for eq in input {
        let eq_res = eq.result;
        if check_eq(eq, &ALL_OPS) {
            res += eq_res;
        }
    }
    res
}

#[cfg(test)]
mod test {
    use super::*;
    static INP: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
";

    #[test]
    fn test_part1() {
        let i = parse_input(INP);
        let res = part1(i);
        assert_eq!(res, 3749);
    }

    #[test]
    fn test_part2() {
        let i = parse_input(INP);
        let res = part2(i);
        assert_eq!(res, 11387);
    }
}
