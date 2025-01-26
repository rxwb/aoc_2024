use clap::Parser;
use std::fmt;
use std::fs;
use std::path::PathBuf;

#[derive(Parser, Debug)]
struct Cli {
    in_path: PathBuf,
}

#[derive(Clone, Debug)]
enum GuardDir {
    Up,
    Down,
    Right,
    Left,
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
    guard_pos: (usize, usize),
    guard_dir: GuardDir,
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut out = "".to_string();
        for (y, row) in self.map.iter().enumerate() {
            for (x, c) in row.iter().enumerate() {
                if self.guard_pos == (x, y) {
                    match self.guard_dir {
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
    let res = part1(&i);
    println!("{res}");
    let res = part2(&i);
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
        guard_pos,
        guard_dir,
    }
}

fn get_tile(map: &[Vec<Tile>], pos: (usize, usize)) -> Tile {
    map[pos.1][pos.0]
}

fn part1(input: &State) -> u64 {
    let mut state = input.clone();
    let max_x = state.map[0].len() - 1;
    let max_y = state.map.len() - 1;
    loop {
        let (x, y) = state.guard_pos;
        let (new_pos, new_dir) = match state.guard_dir {
            GuardDir::Up => {
                if y == 0 {
                    break;
                }
                match get_tile(&state.map, (x, y - 1)) {
                    Tile::Blocked => ((x, y), GuardDir::Right),
                    Tile::Free | Tile::Visited => {
                        state.map[y - 1][x] = Tile::Visited;
                        ((x, y - 1), GuardDir::Up)
                    }
                }
            }
            GuardDir::Down => {
                if y == max_y {
                    break;
                }
                match get_tile(&state.map, (x, y + 1)) {
                    Tile::Blocked => ((x, y), GuardDir::Left),
                    Tile::Free | Tile::Visited => {
                        state.map[y + 1][x] = Tile::Visited;
                        ((x, y + 1), GuardDir::Down)
                    }
                }
            }
            GuardDir::Left => {
                if x == 0 {
                    break;
                }
                match get_tile(&state.map, (x - 1, y)) {
                    Tile::Blocked => ((x, y), GuardDir::Up),
                    Tile::Free | Tile::Visited => {
                        state.map[y][x - 1] = Tile::Visited;
                        ((x - 1, y), GuardDir::Left)
                    }
                }
            }
            GuardDir::Right => {
                if x == max_x {
                    break;
                }
                match get_tile(&state.map, (x + 1, y)) {
                    Tile::Blocked => ((x, y), GuardDir::Down),
                    Tile::Free | Tile::Visited => {
                        state.map[y][x + 1] = Tile::Visited;
                        ((x + 1, y), GuardDir::Right)
                    }
                }
            }
        };
        state.guard_pos = new_pos;
        state.guard_dir = new_dir;
    }

    let mut res = 0;
    for line in state.map {
        for t in line {
            if t == Tile::Visited {
                res += 1;
            }
        }
    }
    res
}

fn part2(_input: &State) -> u64 {
    0
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
        let res = part1(&i);
        assert_eq!(res, 41);
    }

    #[test]
    fn test_part2() {
        let i = parse_input(INP);
        let res = part2(&i);
        assert_eq!(res, 0);
    }
}
