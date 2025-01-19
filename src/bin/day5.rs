use clap::Parser;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::PathBuf;

#[derive(Parser, Debug)]
struct Cli {
    in_path: PathBuf,
}

#[derive(PartialEq)]
enum Ordering {
    Correct,
    Wrong(usize, usize),
}

fn main() {
    let cli = Cli::parse();
    let f = fs::read_to_string(cli.in_path).unwrap();
    let mut i = parse_input(&f);
    let rules = mapify_rules(&i.0);
    let res = part1(&rules, &i.1);
    println!("{res}");
    let res = part2(&rules, &mut i.1);
    println!("{res}");
}

fn parse_input(input: &str) -> (Vec<(u64, u64)>, Vec<Vec<u64>>) {
    let mut rules = vec![];
    let mut pages = vec![];

    let mut step1 = true;
    for line in input.lines() {
        if step1 {
            if line.is_empty() {
                step1 = false;
                continue;
            }
            let mut s = line.split('|');
            let l = s.next().unwrap();
            let r = s.next().unwrap();
            rules.push((l.parse().unwrap(), r.parse().unwrap()));
        } else {
            let p = line.split(',').map(|x| x.parse().unwrap()).collect();
            pages.push(p);
        }
    }

    (rules, pages)
}

fn mapify_rules(rules: &[(u64, u64)]) -> HashMap<u64, Vec<u64>> {
    let mut res = HashMap::new();
    for r in rules {
        res.entry(r.0)
            .and_modify(|x: &mut Vec<u64>| x.push(r.1))
            .or_insert(vec![r.1]);
    }
    res
}

fn correct_ordering(update: &[u64], rules: &HashMap<u64, Vec<u64>>) -> Ordering {
    let mut seen_pages = HashSet::new();
    for (i, x) in update.iter().enumerate() {
        let r = &rules.get(x);
        if r.is_some() {
            let r = r.unwrap();
            for y in r {
                if seen_pages.contains(y) {
                    for (j, z) in update.iter().enumerate() {
                        if y == z {
                            return Ordering::Wrong(i, j);
                        }
                    }
                    unreachable!();
                }
            }
        }
        seen_pages.insert(x);
    }
    Ordering::Correct
}

fn part1(rules: &HashMap<u64, Vec<u64>>, pages: &[Vec<u64>]) -> u64 {
    let mut res = 0;
    for p in pages {
        if correct_ordering(p, rules) == Ordering::Correct {
            let middle = p.len() / 2;
            res += p[middle];
        }
    }
    res
}

fn part2(rules: &HashMap<u64, Vec<u64>>, pages: &mut [Vec<u64>]) -> u64 {
    let mut res = 0;
    for p in pages {
        let mut was_wrong = false;
        loop {
            match correct_ordering(p, rules) {
                Ordering::Correct => break,
                Ordering::Wrong(a, b) => {
                    was_wrong = true;
                    p.swap(a, b)
                }
            }
        }
        if was_wrong {
            let middle = p.len() / 2;
            res += p[middle];
        }
    }
    res
}

#[cfg(test)]
mod test {
    use super::*;
    static INP: &str = "47|53
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
97,13,75,29,47
";

    #[test]
    fn test_parse() {
        let res = parse_input(INP);
        assert_eq!(res.0.len(), 21);
        assert_eq!(res.0[0], (47, 53));
        assert_eq!(res.1.len(), 6);
        assert_eq!(res.1[5], vec![97, 13, 75, 29, 47]);
    }

    #[test]
    fn test_mapify() {
        let (i, _) = parse_input(INP);
        let res = mapify_rules(&i);
        assert_eq!(res[&97], vec![13, 61, 47, 29, 53, 75]);
        assert_eq!(res[&75], vec![29, 53, 47, 61, 13]);
        assert_eq!(res[&47], vec![53, 13, 61, 29]);
        assert_eq!(res[&29], vec![13]);
        assert_eq!(res[&53], vec![29, 13]);
        assert_eq!(res[&61], vec![13, 53, 29]);
    }

    #[test]
    fn test_part1() {
        let i = parse_input(INP);
        let rules = mapify_rules(&i.0);
        let res = part1(&rules, &i.1);
        assert_eq!(res, 143);
    }

    #[test]
    fn test_part2() {
        let mut i = parse_input(INP);
        let rules = mapify_rules(&i.0);
        let res = part2(&rules, &mut i.1);
        assert_eq!(res, 123);
    }
}
