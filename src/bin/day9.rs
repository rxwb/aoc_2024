use clap::Parser;
use std::fs;
use std::path::PathBuf;

#[derive(Parser, Debug)]
struct Cli {
    in_path: PathBuf,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum FileEntry {
    Free,
    File(u32),
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

fn parse_input(input: &str) -> Vec<FileEntry> {
    let mut ret = Vec::new();
    for (i, c) in input.chars().enumerate() {
        if c == '\n' {
            break;
        }
        let i: u32 = i.try_into().unwrap();
        let num = c.to_digit(10).unwrap();
        let e = if (i % 2) == 0 {
            FileEntry::File(i / 2)
        } else {
            FileEntry::Free
        };
        for _ in 0..num {
            ret.push(e);
        }
    }
    ret
}

fn checksum(input: &[FileEntry]) -> usize {
    input
        .iter()
        .enumerate()
        .map(|(i, e)| match e {
            FileEntry::File(f) => i * (*f as usize),
            FileEntry::Free => 0,
        })
        .sum()
}

fn part1(mut input: Vec<FileEntry>) -> usize {
    fn next_free_idx(input: &[FileEntry], start: usize, max: usize) -> Option<usize> {
        (start..max).find(|&i| input[i] == FileEntry::Free)
    }
    let free_idx = 0;
    for read_idx in (0..input.len()).rev() {
        if let Some(free_idx) = next_free_idx(&input, free_idx, read_idx) {
            if input[read_idx] == FileEntry::Free {
                continue;
            }
            input[free_idx] = input[read_idx];
            input[read_idx] = FileEntry::Free;
        } else {
            break;
        }
    }

    checksum(&input)
}

fn part2(_input: Vec<FileEntry>) -> u64 {
    0
}

#[cfg(test)]
mod test {
    use super::*;
    static INP: &str = "2333133121414131402
";

    #[test]
    fn test_part1() {
        let i = parse_input(INP);
        let res = part1(i);
        assert_eq!(res, 1928);
    }

    #[test]
    fn test_part2() {
        let i = parse_input(INP);
        let res = part2(i);
        assert_eq!(res, 0);
    }
}
