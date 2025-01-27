use clap::Parser;
use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;

#[derive(Parser, Debug)]
struct Cli {
    in_path: PathBuf,
}

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
    height: u8,
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

fn parse_input(input: &str) -> Vec<Vec<u8>> {
    let mut res = Vec::new();
    for line in input.lines() {
        res.push(
            line.chars()
                .filter_map(|c| c.to_digit(10))
                .map(|x| x as u8)
                .collect(),
        );
    }

    res
}

fn find_neighbors(map: &[Vec<u8>], p: Point) -> Vec<Point> {
    let mut res = Vec::with_capacity(4);
    let height = p.height + 1;
    let x = p.x;
    let y = p.y;
    if (x > 0) && (map[y][x - 1]) == height {
        res.push(Point {
            x: x - 1,
            y,
            height,
        });
    }
    if (y > 0) && (map[y - 1][x]) == height {
        res.push(Point {
            x,
            y: y - 1,
            height,
        });
    }
    if let Some(h) = map[y].get(x + 1) {
        if *h == height {
            res.push(Point {
                x: x + 1,
                y,
                height,
            })
        }
    }
    if let Some(line) = map.get(y + 1) {
        if line[x] == height {
            res.push(Point {
                x,
                y: y + 1,
                height,
            })
        }
    }

    res
}

fn part1(input: &[Vec<u8>]) -> usize {
    let mut res = 0;

    for (y, line) in input.iter().enumerate() {
        for (x, &height) in line.iter().enumerate() {
            if height == 0 {
                let mut to_check = Vec::new();
                to_check.extend(find_neighbors(input, Point { x, y, height }));
                let mut summits = HashSet::new();
                while let Some(p) = to_check.pop() {
                    if p.height == 9 {
                        summits.insert((p.x, p.y));
                    } else {
                        to_check.extend(find_neighbors(input, p));
                    }
                }
                res += summits.len();
            }
        }
    }

    res
}

fn part2(_input: &[Vec<u8>]) -> u64 {
    0
}

#[cfg(test)]
mod test {
    use super::*;
    static INP: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
";

    #[test]
    fn test_part1() {
        let i = parse_input(INP);
        let res = part1(&i);
        assert_eq!(res, 36);
    }

    #[test]
    fn test_part2() {
        let i = parse_input(INP);
        let res = part2(&i);
        assert_eq!(res, 0);
    }
}
