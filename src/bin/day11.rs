use clap::Parser;
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
    let res = part1(i.clone());
    println!("{res}");
    let res = part2(i);
    println!("{res}");
}

fn parse_input(input: &str) -> Vec<u64> {
    input.split(' ').filter_map(|x| x.trim().parse().ok()).collect()
}

fn part1(mut input: Vec<u64>) -> usize {
    for _ in 0..25 {
        let mut next = Vec::with_capacity(input.len() * 2);
        for stone in input.into_iter() {
            let st = stone.to_string();
            if stone == 0 {
                next.push(1);
            } else if (st.len() % 2) == 0 {
                let mid = st.len() / 2;
                let left = &st[0..mid];
                let right = &st[mid..st.len()];
                next.extend([left, right].map(|x| x.parse::<u64>().unwrap()));
            } else {
                next.push(stone * 2024);
            }
        }
        input = next;
    }
    input.len()
}

fn part2(mut _input: Vec<u64>) -> usize {
    0
}

#[cfg(test)]
mod test {
    use super::*;
    static INP: &str = "125 17
";

    #[test]
    fn test_part1() {
        let i = parse_input(INP);
        let res = part1(i);
        assert_eq!(res, 55312);
    }
}
