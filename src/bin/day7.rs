use clap::Parser;
use itertools::Itertools;
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

#[derive(Clone, Debug)]
enum Operator {
    Add,
    Mul,
}

const ADD_MUL: [Operator; 2] = [Operator::Add, Operator::Mul];

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

fn part1(input: Vec<Equation>) -> i64 {
    let mut res = 0;
    for eq in input {
        let num_ops = eq.numbers.len() - 1;
        for ops in (0..num_ops).map(|_| ADD_MUL).multi_cartesian_product() {
            let mut nums = eq.numbers.iter();
            let mut left = nums.next().unwrap();
            let mut acc = 0;
            for o in ops.iter() {
                let right = nums.next().unwrap();
                acc = match o {
                    Operator::Add => left + right,
                    Operator::Mul => left * right,
                };
                left = &acc;
            }
            if acc == eq.result {
                res += acc;
                break;
            }
        }
    }
    res
}

fn part2(_input: Vec<Equation>) -> u64 {
    0
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
        assert_eq!(res, 0);
    }
}
