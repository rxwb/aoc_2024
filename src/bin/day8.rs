use clap::Parser;
use itertools::Itertools;
use std::cmp;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::fs;
use std::path::PathBuf;

#[derive(Parser, Debug)]
struct Cli {
    in_path: PathBuf,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn main() {
    let cli = Cli::parse();
    let f = fs::read_to_string(cli.in_path).unwrap();
    let (i, max) = parse_input(&f);
    let res = part1(&i, max);
    println!("{res}");
    let res = part2(&i);
    println!("{res}");
}

fn parse_input(input: &str) -> (HashMap<char, Vec<Point>>, Point) {
    let mut res: HashMap<char, Vec<Point>> = HashMap::new();
    let mut max_x = 0;
    let mut max_y = 0;

    for (y, line) in input.lines().enumerate() {
        max_x = cmp::max(max_x, line.len());
        for (x, ch) in line.chars().enumerate() {
            if ch == '.' {
                continue;
            } else {
                res.entry(ch).or_default().push(Point { x, y });
            }
        }
        max_y = y
    }

    (res, Point { x: max_x, y: max_y })
}

fn antinodes(a1: Point, a2: Point, max: Point, max_multiplicator: usize) -> Vec<Point> {
    fn get_node(a1: Point, a2: Point, max: Point, max_multiplicator: usize) -> Vec<Point> {
        let min_x = cmp::min(a1.x, a2.x);
        let max_x = cmp::max(a1.x, a2.x);
        let min_y = cmp::min(a1.y, a2.y);
        let max_y = cmp::max(a1.y, a2.y);
        let x_diff_org = max_x - min_x;
        let y_diff_org = max_y - min_y;
        let mut res = Vec::new();
        for i in 1..=max_multiplicator {
            let x_diff = x_diff_org * i;
            let y_diff = y_diff_org * i;
            if a1.x < a2.x {
                // antinode left of a1
                if a1.x >= x_diff {
                    if a1.y < a2.y {
                        // antinode above a1
                        if a1.y >= y_diff {
                            res.push(Point {
                                x: a1.x - x_diff,
                                y: a1.y - y_diff,
                            });
                        }
                    } else {
                        // antinode below a1
                        if a1.y <= max.y - y_diff {
                            res.push(Point {
                                x: a1.x - x_diff,
                                y: a1.y + y_diff,
                            });
                        } else {
                            break;
                        }
                    }
                } else {
                    break;
                }
            } else {
                // antinode right of a1
                if a1.x < max.x - x_diff {
                    if a1.y < a2.y {
                        // antinode above a1
                        if a1.y >= y_diff {
                            res.push(Point {
                                x: a1.x + x_diff,
                                y: a1.y - y_diff,
                            });
                        }
                    } else {
                        // antinode below a1
                        if a1.y <= max.y - y_diff {
                            res.push(Point {
                                x: a1.x + x_diff,
                                y: a1.y + y_diff,
                            });
                        } else {
                            break;
                        }
                    }
                } else {
                    break;
                }
            }
        }
        res
    }

    let mut res = Vec::new();
    res.extend(get_node(a1, a2, max, max_multiplicator));
    res.extend(get_node(a2, a1, max, max_multiplicator));

    res
}

fn part1(input: &HashMap<char, Vec<Point>>, max: Point) -> usize {
    let mut res = HashSet::new();
    for antennas in input.values() {
        for pair in antennas.iter().combinations(2) {
            for node in antinodes(*pair[0], *pair[1], max, 1) {
                res.insert(node);
            }
        }
    }
    res.len()
}

fn part2(_input: &HashMap<char, Vec<Point>>) -> u64 {
    0
}

#[cfg(test)]
mod test {
    use super::*;
    static INP: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
";

    #[test]
    fn test_part1() {
        let (i, max) = parse_input(INP);
        let res = part1(&i, max);
        assert_eq!(res, 14);
    }

    #[test]
    fn test_part2() {
        let (i, _max) = parse_input(INP);
        let res = part2(&i);
        assert_eq!(res, 0);
    }
}
