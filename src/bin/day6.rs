use clap::Parser;
use itertools::iproduct;
use std::collections::HashSet;
use std::fmt;
use std::fs;
use std::path::PathBuf;

#[derive(Parser, Debug)]
struct Cli {
    in_path: PathBuf,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
enum GuardDir {
    Up,
    Down,
    Right,
    Left,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct GuardState {
    dir: GuardDir,
    pos: (usize, usize),
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Tile {
    Free,
    Blocked,
    Visited,
}

#[derive(Clone, Debug)]
struct State {
    map: Vec<Vec<Tile>>,
    guard: GuardState,
}

impl State {
    fn get_tile(&self, x: usize, y: usize) -> Tile {
        *self
            .map
            .get(y)
            .expect("Invalid y {y}")
            .get(x)
            .expect("Invalid x {x}")
    }

    fn set_tile(&mut self, x: usize, y: usize, tile: Tile) {
        *self
            .map
            .get_mut(y)
            .expect("Invalid y {y}")
            .get_mut(x)
            .expect("Invalid x {x}") = tile;
    }

    fn step(&mut self) -> Option<GuardState> {
        let max_x = self.map[0].len() - 1;
        let max_y = self.map.len() - 1;
        let (x, y) = self.guard.pos;
        let (new_pos, new_dir) = match self.guard.dir {
            GuardDir::Up => {
                if y == 0 {
                    return None;
                }
                match self.get_tile(x, y - 1) {
                    Tile::Blocked => ((x, y), GuardDir::Right),
                    Tile::Free | Tile::Visited => ((x, y - 1), GuardDir::Up),
                }
            }
            GuardDir::Down => {
                if y == max_y {
                    return None;
                }
                match self.get_tile(x, y + 1) {
                    Tile::Blocked => ((x, y), GuardDir::Left),
                    Tile::Free | Tile::Visited => ((x, y + 1), GuardDir::Down),
                }
            }
            GuardDir::Left => {
                if x == 0 {
                    return None;
                }
                match self.get_tile(x - 1, y) {
                    Tile::Blocked => ((x, y), GuardDir::Up),
                    Tile::Free | Tile::Visited => ((x - 1, y), GuardDir::Left),
                }
            }
            GuardDir::Right => {
                if x == max_x {
                    return None;
                }
                match self.get_tile(x + 1, y) {
                    Tile::Blocked => ((x, y), GuardDir::Down),
                    Tile::Free | Tile::Visited => ((x + 1, y), GuardDir::Right),
                }
            }
        };
        self.set_tile(new_pos.0, new_pos.1, Tile::Visited);
        self.guard.pos = new_pos;
        self.guard.dir = new_dir;

        Some(self.guard)
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut out = "".to_string();
        for (y, row) in self.map.iter().enumerate() {
            for (x, c) in row.iter().enumerate() {
                if self.guard.pos == (x, y) {
                    match self.guard.dir {
                        GuardDir::Up => out.push('^'),
                        GuardDir::Down => out.push('v'),
                        GuardDir::Left => out.push('<'),
                        GuardDir::Right => out.push('>'),
                    }
                } else {
                    match c {
                        Tile::Free => out.push('.'),
                        Tile::Blocked => out.push('#'),
                        Tile::Visited => out.push('X'),
                    }
                }
            }
            out.push('\n');
        }
        write!(f, "{out}")
    }
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

fn parse_input(input: &str) -> State {
    let mut map = vec![];
    let mut guard_pos = (0, 0);
    let mut guard_dir = GuardDir::Up;

    for (y, line) in input.lines().enumerate() {
        let mut row = vec![];
        for (x, c) in line.chars().enumerate() {
            match c {
                '.' => row.push(Tile::Free),
                '#' => row.push(Tile::Blocked),
                '^' => {
                    row.push(Tile::Visited);
                    guard_dir = GuardDir::Up;
                    guard_pos = (x, y);
                }
                'v' => {
                    row.push(Tile::Visited);
                    guard_dir = GuardDir::Down;
                    guard_pos = (x, y);
                }
                '>' => {
                    row.push(Tile::Visited);
                    guard_dir = GuardDir::Right;
                    guard_pos = (x, y);
                }
                '<' => {
                    row.push(Tile::Visited);
                    guard_dir = GuardDir::Left;
                    guard_pos = (x, y);
                }
                _ => unreachable!(),
            }
        }
        map.push(row);
    }

    State {
        map,
        guard: GuardState {
            pos: guard_pos,
            dir: guard_dir,
        },
    }
}

fn part1(mut input: State) -> u64 {
    while input.step().is_some() {}

    let mut res = 0;
    for line in input.map {
        for t in line {
            if t == Tile::Visited {
                res += 1;
            }
        }
    }
    res
}

fn part2(input: State) -> u64 {
    let max_x = input.map[0].len();
    let max_y = input.map.len();

    let mut res = 0;

    for (y, x) in iproduct!(0..max_y, 0..max_x) {
        let mut state = input.clone();
        if state.get_tile(x, y) == Tile::Free {
            state.set_tile(x, y, Tile::Blocked);
        } else {
            continue;
        }
        let mut seen = HashSet::new();
        while let Some(guard) = state.step() {
            if seen.contains(&guard) {
                res += 1;
                break;
            }
            seen.insert(guard);
        }
    }

    res
}

#[cfg(test)]
mod test {
    use super::*;
    static INP: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";

    #[test]
    fn test_part1() {
        let i = parse_input(INP);
        let res = part1(i);
        assert_eq!(res, 41);
    }

    #[test]
    fn test_part2() {
        let i = parse_input(INP);
        let res = part2(i);
        assert_eq!(res, 6);
    }
}
