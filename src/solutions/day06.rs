use crate::*;
use anyhow::{Context, Error, Result};
use rustc_hash::FxHashSet;

pub fn solve(part: u8, input: &'static str) -> Result<Solution, Error> {
    #![allow(unused)]
    let input = if input.len() > 0 {
        input
    } else {
        include_str!("../../data/06.input")
    };

    let w = input.lines().next().context("no 1st line")?.len();
    let mut data = input.chars().filter(|x| *x != '\n').collect::<Vec<_>>();
    let n = data.len();
    let data = data;
    assert!(n % w == 0);
    let h = n / w;

    // easy bounds
    let w = w as isize;
    let h = h as isize;

    let idx = |row, col| (row * w + col) as usize;
    let valid = |row, col| row >= 0 && row < h && col >= 0 && col < w;

    let start = data
        .iter()
        .enumerate()
        .filter_map(|(idx, x)| if *x == '^' { Some(idx) } else { None })
        .next()
        .context("no ^ found in data")? as isize;
    let (start_row, start_col) = (start / w, start % w);

    let move_op = |dir, row, col| {
        let (row, col) = match dir {
            0 => (row - 1, col), // north
            1 => (row, col + 1), // east
            2 => (row + 1, col), // south
            3 => (row, col - 1), // west
            _ => unreachable!(),
        };
        if valid(row, col) {
            Some((row, col))
        } else {
            None
        }
    };
    let turn = |dir| (dir + 1) % 4;
    let mut dir = 0;
    let (mut row, mut col) = (start_row, start_col);

    if part == 1 {
        let mut path = FxHashSet::default();
        'path: loop {
            path.insert((row, col));
            let Some((row_next, col_next)) = move_op(dir, row, col) else {
                break;
            };
            match data[idx(row_next, col_next)] {
                '#' => dir = turn(dir),
                _ => (row, col) = (row_next, col_next),
            }
        }
        Ok(Solution::U64(path.len() as u64))
    } else {
        let mut obstructions = FxHashSet::default();
        let mut path: Vec<bool> = Default::default();
        path.resize(n, false);

        let mut dirs: [Vec<bool>; 4] = Default::default();
        for i in &mut dirs {
            i.resize(n, false);
        }
        let mut dirs_loop: [Vec<bool>; 4] = Default::default();
        for i in &mut dirs_loop {
            i.resize(n, false);
        }
        'path: loop {
            path[idx(row, col)] = true;
            dirs[dir][idx(row, col)] = true;
            let Some((row_next, col_next)) = move_op(dir, row, col) else {
                break;
            };
            if data[idx(row_next, col_next)] == '#' {
                dir = turn(dir);
            } else {
                // test infinite loop case
                // XXX impossible to put an obstacle on previous position along the path
                if !path[idx(row_next, col_next)] {
                    let mut dir = turn(dir);
                    let (mut row, mut col) = (row, col);
                    for i in 0..4 {
                        for j in 0..n {
                            dirs_loop[i][j] = dirs[i][j];
                        }
                    }
                    loop {
                        dirs_loop[dir][idx(row, col)] = true;
                        let Some((row_loop_next, col_loop_next)) = move_op(dir, row, col) else {
                            break;
                        };
                        if data[idx(row_loop_next, col_loop_next)] == '#'
                            || (row_loop_next == row_next && col_loop_next == col_next)
                        {
                            dir = turn(dir);
                        } else {
                            (row, col) = (row_loop_next, col_loop_next);
                        }
                        if dirs_loop[dir][idx(row, col)] {
                            obstructions.insert((row_next, col_next));
                            break;
                        }
                    }
                }
                // usual move
                (row, col) = (row_next, col_next);
            }
        }

        Ok(Solution::U64(obstructions.len() as u64))
    }
}

pub fn sol(part: u8, input: &'static str) -> Solution {
    solve(part, input).expect("solve test")
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn part_1_sample() {
        assert_eq!(sol(1, SAMPLE), Solution::U64(41));
    }

    #[test]
    fn part_1() {
        assert_eq!(sol(1, ""), Solution::U64(5030));
    }

    #[test]
    fn part_2_sample() {
        assert_eq!(sol(2, SAMPLE), Solution::U64(6));
    }

    #[test]
    fn part_2() {
        assert_eq!(sol(2, ""), Solution::U64(1928));
    }
}
