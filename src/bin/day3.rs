use clap::Parser;
use regex::Regex;
use std::fs;
use std::path::PathBuf;

#[derive(Parser, Debug)]
struct Cli {
    in_path: PathBuf,
}

#[derive(Eq, PartialEq, Debug)]
enum Ins {
    Mul(u64, u64),
    Do,
    Dont,
}

fn main() {
    let cli = Cli::parse();
    let f = fs::read_to_string(cli.in_path).unwrap();
    let i = parse_input1(&f);
    let res = part1(&i);
    println!("{res}");
    let i = parse_input2(&f);
    let res = part2(&i);
    println!("{res}");
}

fn parse_input1(input: &str) -> Vec<(u64, u64)> {
    let re = Regex::new(r"mul\(([[:digit:]]+),([[:digit:]]+)\)").unwrap();
    let mut res = vec![];
    for (_, [l, r]) in re.captures_iter(input).map(|c| c.extract()) {
        res.push((l.parse().unwrap(), r.parse().unwrap()));
    }
    res
}

fn parse_input2(input: &str) -> Vec<Ins> {
    let re = Regex::new(r"mul\(([[:digit:]]+),([[:digit:]]+)\)|(d)(o)\(\)|do(n)'(t)\(\)").unwrap();
    let mut res = vec![];
    for mat in re.captures_iter(input) {
        let (all, [l, r]) = mat.extract();

        if all.starts_with('m') {
            res.push(Ins::Mul(l.parse().unwrap(), r.parse().unwrap()));
        } else if r == "t" {
            res.push(Ins::Dont);
        } else {
            res.push(Ins::Do);
        }
    }
    res
}

fn part1(input: &[(u64, u64)]) -> u64 {
    input.iter().map(|x| x.0 * x.1).sum()
}

fn part2(input: &[Ins]) -> u64 {
    let mut enabled = true;
    let mut res = 0;
    for ins in input {
        match ins {
            Ins::Do => enabled = true,
            Ins::Dont => enabled = false,
            Ins::Mul(l, r) => res += enabled as u64 * l * r,
        }
    }
    res
}

#[cfg(test)]
mod test {
    use super::*;
    static INP1: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    static INP2: &str = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn test_parse1() {
        let res = parse_input1(INP1);
        assert_eq!(res.len(), 4);
        assert_eq!(res[0], (2, 4));
        assert_eq!(res[1], (5, 5));
        assert_eq!(res[2], (11, 8));
        assert_eq!(res[3], (8, 5));
    }

    #[test]
    fn test_parse2() {
        let res = parse_input2(INP2);
        assert_eq!(res.len(), 6);
        assert_eq!(res[0], Ins::Mul(2, 4));
        assert_eq!(res[1], Ins::Dont);
        assert_eq!(res[2], Ins::Mul(5, 5));
        assert_eq!(res[3], Ins::Mul(11, 8));
        assert_eq!(res[4], Ins::Do);
        assert_eq!(res[5], Ins::Mul(8, 5));
    }

    #[test]
    fn test_part1() {
        let i = parse_input1(INP1);
        let res = part1(&i);
        assert_eq!(res, 161);
    }

    #[test]
    fn test_part2() {
        let i = parse_input2(INP2);
        let res = part2(&i);
        assert_eq!(res, 48);
    }
}
