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

#[derive(Debug)]
struct Span {
    entry: FileEntry,
    index: usize,
    length: usize,
}

fn main() {
    let cli = Cli::parse();
    let f = fs::read_to_string(cli.in_path).unwrap();
    let i = parse_input_to_vec(&f);
    let res = part1(i);
    println!("{res}");
    let (files, empties) = parse_input_to_spans(&f);
    let res = part2(files, empties);
    println!("{res}");
}

fn parse_input_to_vec(input: &str) -> Vec<FileEntry> {
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

fn parse_input_to_spans(input: &str) -> (Vec<Span>, Vec<Span>) {
    let mut file_span = Vec::new();
    let mut empty_span = Vec::new();
    let mut index = 0;

    for (i, c) in input.chars().enumerate() {
        if c == '\n' {
            break;
        }
        let i: u32 = i.try_into().unwrap();
        let length = c.to_digit(10).unwrap() as usize;
        if (i % 2) == 0 {
            file_span.push(Span {
                entry: FileEntry::File(i / 2),
                index,
                length,
            })
        } else {
            empty_span.push(Span {
                entry: FileEntry::Free,
                index,
                length,
            })
        }
        index += length;
    }

    (file_span, empty_span)
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

// TODO: rework to also work on vectos of Span and benchamrk
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

fn part2(mut files: Vec<Span>, mut empties: Vec<Span>) -> usize {
    for file in files.iter_mut().rev() {
        if let Some(idx) = empties
            .iter()
            .position(|e| (e.index < file.index) && (e.length >= file.length))
        {
            file.index = empties[idx].index;
            empties[idx].length -= file.length;
            empties[idx].index += file.length;
            // TODO: should we drop empty spans with length 0? -> in that case VecDeque?
        }
    }

    let mut sum = 0;
    for f in files {
        if let FileEntry::File(id) = f.entry {
            for i in 0..(f.length) {
                sum += (f.index + i) * id as usize;
            }
        }
    }
    sum
}

#[cfg(test)]
mod test {
    use super::*;
    static INP: &str = "2333133121414131402
";

    #[test]
    fn test_part1() {
        let i = parse_input_to_vec(INP);
        let res = part1(i);
        assert_eq!(res, 1928);
    }

    #[test]
    fn test_part2() {
        let (files, empties) = parse_input_to_spans(INP);
        let res = part2(files, empties);
        assert_eq!(res, 2858);
    }
}
