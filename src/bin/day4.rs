use clap::Parser;
use std::fs;
use std::path::PathBuf;

#[derive(Parser, Debug)]
struct Cli {
    in_path: PathBuf,
}

#[derive(PartialEq, Debug)]
enum Dir {
    Pos,
    Zer,
    Neg,
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

fn parse_input(input: &str) -> Vec<String> {
    input.lines().map(|i| i.into()).collect()
}

fn part1(input: &[String]) -> u64 {
    const PATTERN: &str = "XMAS";
    let mut res = 0;
    let num_lines = input.len();
    let line_len = input[0].len();
    for (i, line) in input.iter().enumerate() {
        for (j, ch) in line.chars().enumerate() {
            if ch == PATTERN.chars().next().unwrap() {
                let strides = vec![
                    (Dir::Neg, Dir::Zer),
                    (Dir::Pos, Dir::Zer),
                    (Dir::Zer, Dir::Neg),
                    (Dir::Zer, Dir::Pos),
                    (Dir::Neg, Dir::Neg),
                    (Dir::Neg, Dir::Pos),
                    (Dir::Pos, Dir::Neg),
                    (Dir::Pos, Dir::Pos),
                ];
                for stride in strides {
                    let enough_x = (stride.0 == Dir::Zer)
                        || ((stride.0 == Dir::Neg) && (j >= PATTERN.len() - 1))
                        || ((stride.0 == Dir::Pos) && (j <= line_len - PATTERN.len()));
                    let enough_y = (stride.1 == Dir::Zer)
                        || ((stride.1 == Dir::Neg) && (i >= PATTERN.len() - 1))
                        || ((stride.1 == Dir::Pos) && (i <= num_lines - PATTERN.len()));
                    if enough_x && enough_y {
                        let mut corr = 1;
                        let mut it = PATTERN.chars().enumerate();
                        _ = it.next();
                        for (k, p) in it {
                            let x = match stride.0 {
                                Dir::Neg => j - k,
                                Dir::Zer => j,
                                Dir::Pos => j + k,
                            };
                            let y = match stride.1 {
                                Dir::Neg => i - k,
                                Dir::Zer => i,
                                Dir::Pos => i + k,
                            };
                            let chk = input[y].chars().nth(x);
                            if chk == Some(p) {
                                corr += 1;
                            } else {
                                break;
                            }
                        }
                        if corr == PATTERN.len() {
                            res += 1;
                        }
                    }
                }
            }
        }
    }
    res
}

fn part2(input: &[String]) -> u64 {
    let mut res = 0;
    let num_lines = input.len();
    let line_len = input[0].len();
    for i in 1..(num_lines - 1) {
        for j in 1..(line_len - 1) {
            if input[i].chars().nth(j).unwrap() == 'A' {
                let top_left = input[i - 1].chars().nth(j - 1).unwrap();
                let top_right = input[i - 1].chars().nth(j + 1).unwrap();
                let bot_left = input[i + 1].chars().nth(j - 1).unwrap();
                let bot_right = input[i + 1].chars().nth(j + 1).unwrap();
                let diag1 = ((top_left == 'M') && (bot_right == 'S'))
                    || ((top_left == 'S') && (bot_right == 'M'));
                let diag2 = ((bot_left == 'M') && (top_right == 'S'))
                    || ((bot_left == 'S') && (top_right == 'M'));
                if diag1 && diag2 {
                    res += 1;
                }
            }
        }
    }

    res
}

#[cfg(test)]
mod test {
    use super::*;
    static INP: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";
    #[test]
    fn test_parse() {
        let parsed = parse_input(INP);
        assert_eq!(parsed.len(), 10);
        assert_eq!(parsed[0], "MMMSXXMASM");
        assert_eq!(parsed[1], "MSAMXMSMSA");
        assert_eq!(parsed[2], "AMXSXMAAMM");
        assert_eq!(parsed[3], "MSAMASMSMX");
        assert_eq!(parsed[4], "XMASAMXAMM");
        assert_eq!(parsed[5], "XXAMMXXAMA");
        assert_eq!(parsed[6], "SMSMSASXSS");
        assert_eq!(parsed[7], "SAXAMASAAA");
        assert_eq!(parsed[8], "MAMMMXMMMM");
        assert_eq!(parsed[9], "MXMXAXMASX");
    }

    #[test]
    fn test_part1() {
        let i = parse_input(INP);
        let res = part1(&i);
        assert_eq!(res, 18);
    }

    #[test]
    fn test_part1_min() {
        let inp = "XMAS
MxxA
AxxM
SAMX
";
        let i = parse_input(&inp);
        let res = part1(&i);
        assert_eq!(res, 4);
    }

    #[test]
    fn test_part2() {
        let i = parse_input(INP);
        let res = part2(&i);
        assert_eq!(res, 9);
    }
}
