use clap::Parser;
use std::cmp::{max, min};
use std::collections::HashMap;
use std::fs;
use std::iter::zip;
use std::path::PathBuf;

#[derive(Parser, Debug)]
struct Cli {
    in_path: PathBuf,
}

fn main() {
    let cli = Cli::parse();
    let f = fs::read_to_string(cli.in_path).unwrap();
    let (mut left, mut right) = parse_input(&f);
    let res = part1(&mut left, &mut right);
    println!("{res}");
    let res = part2(&left, &right);
    println!("{res}");
}

fn parse_input(input: &str) -> (Vec<u64>, Vec<u64>) {
    let mut left = vec![];
    let mut right = vec![];
    for line in input.lines() {
        let mut it = line.split_whitespace();
        left.push(it.next().unwrap().parse().unwrap());
        right.push(it.next().unwrap().parse().unwrap());
    }
    (left, right)
}

fn part1(left: &mut Vec<u64>, right: &mut Vec<u64>) -> u64 {
    left.sort();
    right.sort();
    zip(left, right)
        .map(|x| max(*x.0, *x.1) - min(*x.0, *x.1))
        .sum()
}

fn part2(left: &[u64], right: &[u64]) -> u64 {
    let mut ctr = HashMap::new();
    for x in right.iter() {
        ctr.entry(x).and_modify(|c| *c += 1).or_insert(1);
    }
    let mut res = 0u64;
    for x in left.iter() {
        res += x * ctr.get(x).unwrap_or(&0);
    }
    res
}

#[cfg(test)]
mod test {
    use super::*;
    static INP: &str = "3   4
4   3
2   5
1   3
3   9
3   3
";

    #[test]
    fn test_part1() {
        let (mut l, mut r) = parse_input(INP);
        let res = part1(&mut l, &mut r);
        assert_eq!(res, 11);
    }

    #[test]
    fn test_part2() {
        let (l, r) = parse_input(INP);
        let res = part2(&l, &r);
        assert_eq!(res, 31);
    }
}
