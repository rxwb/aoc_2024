use clap::Parser;
use std::cmp::Ordering;
use std::fs;
use std::path::PathBuf;

#[derive(Parser, Debug)]
struct Cli {
    in_path: PathBuf,
}

fn main() {
    let cli = Cli::parse();
    let f = fs::read_to_string(cli.in_path).unwrap();
    let i = parse_input(&f);
    let res = part1(&i);
    println!("{res}");
    let res = part2(&i);
    println!("{res}");
}

fn parse_input(input: &str) -> Vec<Vec<i64>> {
    let mut res = vec![];
    for line in input.lines() {
        let mut r = vec![];
        for num in line.split_whitespace() {
            r.push(num.parse().unwrap());
        }
        res.push(r);
    }
    res
}

fn is_safe(line: &[i64], skip: usize) -> bool {
    let mut it = line.iter().enumerate();
    let mut inc = true;
    let mut dec = true;
    if skip == 0 {
        let _ = it.next();
    }
    let (_, mut prev) = it.next().unwrap();
    for (i, curr) in it {
        if i == skip {
            continue;
        }
        let diff = curr - prev;
        if diff.abs() > 3 {
            return false;
        }
        match diff.cmp(&0) {
            Ordering::Greater => dec = false,
            Ordering::Less => inc = false,
            Ordering::Equal => return false,
        }
        prev = curr;
    }
    inc || dec
}

fn part1(input: &[Vec<i64>]) -> u64 {
    input.iter().map(|x| is_safe(x, usize::MAX) as u64).sum()
}

fn part2(input: &[Vec<i64>]) -> u64 {
    let mut res = 0;
    for line in input {
        let l = line.len();
        for i in 0..l {
            if is_safe(line, i) {
                res += 1;
                break;
            }
        }
    }
    res
}

#[cfg(test)]
mod test {
    use super::*;
    static INP: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";
    #[test]
    fn test_parse() {
        let res = parse_input(INP);
        assert_eq!(res.len(), 6);
        assert_eq!(res[0], vec![7, 6, 4, 2, 1]);
        assert_eq!(res[1], vec![1, 2, 7, 8, 9]);
        assert_eq!(res[2], vec![9, 7, 6, 2, 1]);
        assert_eq!(res[3], vec![1, 3, 2, 4, 5]);
        assert_eq!(res[4], vec![8, 6, 4, 4, 1]);
        assert_eq!(res[5], vec![1, 3, 6, 7, 9]);
    }

    #[test]
    fn test_is_safe() {
        let i = parse_input(INP);
        let res: Vec<bool> = i.iter().map(|x| is_safe(x, usize::MAX)).collect();
        assert_eq!(res[0], true);
        assert_eq!(res[1], false);
        assert_eq!(res[2], false);
        assert_eq!(res[3], false);
        assert_eq!(res[4], false);
        assert_eq!(res[5], true);
    }

    #[test]
    fn test_is_safe_skipped() {
        let i = parse_input(INP);
        let res: Vec<bool> = vec![is_safe(&i[3], 1), is_safe(&i[4], 2)];
        assert_eq!(res[0], true);
        assert_eq!(res[1], true);
    }

    #[test]
    fn test_part1() {
        let i = parse_input(INP);
        let res = part1(&i);
        assert_eq!(res, 2);
    }

    #[test]
    fn test_part2() {
        let i = parse_input(INP);
        let res = part2(&i);
        assert_eq!(res, 4);
    }
}
