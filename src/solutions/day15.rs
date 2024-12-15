use crate::*;
use amplify_num::posit;
use anyhow::{Context, Error, Result};
use itertools::Itertools;
use rustc_hash::FxHashSet;

pub fn solve(part: u8, input: &'static str) -> Result<Solution, Error> {
    #![allow(unused)]
    let input = if input.len() > 0 {
        input
    } else {
        include_str!("../../data/15.input")
    };

    let w = input.lines().next().context("no 1st line")?.len();
    let h = input
        .lines()
        .enumerate()
        .filter_map(|(idx, line)| if line.len() > 0 { None } else { Some(idx) })
        .next()
        .context("no empty line to stop scan")?;
    let n = w * h;
    let mut data = input
        .chars()
        .filter(|x| *x != '\n')
        .take(n)
        .collect::<Vec<_>>();
    assert!(n % w == 0);
    let (w, h) = (w as isize, h as isize);

    let go = |row, col, op| match op {
        '<' => (row, col - 1),
        '>' => (row, col + 1),
        '^' => (row - 1, col),
        'v' => (row + 1, col),
        _ => panic!("invalid movement {op}"),
    };

    if part == 1 {
        let idx = |row, col| (row * w + col) as usize;
        let start = data
            .iter()
            .enumerate()
            .filter_map(|(idx, x)| if *x == '@' { Some(idx) } else { None })
            .next()
            .context("no @ found in data")? as isize;
        let (start_row, start_col) = (start / w, start % w);
        let (mut row, mut col) = (start_row, start_col);

        'op: for (id, op) in input.chars().filter(|x| *x != '\n').skip(n).enumerate() {
            let (row_target, col_target) = go(row, col, op);
            let target = data[idx(row_target, col_target)];
            // for row in 0..h {
            //     for col in 0..w {
            //         print!("{}", data[idx(row, col)]);
            //     }
            //     println!();
            // }
            // println!("mov {id}:{op}({target})");
            match target {
                '#' => (),
                '.' | '@' => (row, col) = (row_target, col_target),
                'O' => {
                    let (mut row_shift, mut col_shift) = (row_target, col_target);
                    loop {
                        (row_shift, col_shift) = go(row_shift, col_shift, op);
                        let shift = data[idx(row_shift, col_shift)];
                        match shift {
                            '#' => continue 'op,
                            '.' | '@' => {
                                data.swap(idx(row_target, col_target), idx(row_shift, col_shift));
                                (row, col) = (row_target, col_target);
                                continue 'op;
                            }
                            'O' => (),
                            _ => panic!("invalid data {shift}"),
                        };
                    }
                }
                _ => panic!("invalid data {target}"),
            }
        }

        let gps = |row, col| 100 * row + col;
        let mut res = 0;
        for row in 0..h {
            for col in 0..w {
                if data[idx(row, col)] == 'O' {
                    res += gps(row, col);
                }
            }
        }

        Ok(Solution::U64(res as u64))
    } else {
        let mut data2 = Vec::with_capacity(n * 2);
        for c in &data {
            match c {
                '#' => {
                    data2.push('#');
                    data2.push('#');
                }
                '.' => {
                    data2.push('.');
                    data2.push('.');
                }
                'O' => {
                    data2.push('[');
                    data2.push(']');
                }
                '@' => {
                    data2.push('@');
                    data2.push('.');
                }
                _ => panic!("invalid data"),
            }
        }
        let mut data = data2;
        let n = 2 * n;
        let w = 2 * w;

        let idx = |row, col| (row * w + col) as usize;
        let start = data
            .iter()
            .enumerate()
            .filter_map(|(idx, x)| if *x == '@' { Some(idx) } else { None })
            .next()
            .context("no @ found in data")? as isize;
        let (start_row, start_col) = (start / w, start % w);
        let (mut row, mut col) = (start_row, start_col);

        let (mut cands, mut cands_next) = (Vec::new(), Vec::new());
        let mut all_cands = Vec::new();

        'op: for (id, op) in input.chars().filter(|x| *x != '\n').skip(n / 2).enumerate() {
            let (row_target, col_target) = go(row, col, op);
            let target = idx(row_target, col_target);
            // for row in 0..h {
            //     for col in 0..w {
            //         print!("{}", data[idx(row, col)]);
            //     }
            //     println!();
            // }
            // println!("mov {id}:{op}({})", data[target]);
            match data[target] {
                '.' => {
                    data[idx(row, col)] = '.';
                    data[target] = '@';
                    (row, col) = (row_target, col_target);
                }
                ']' if op == '<' => {
                    let (mut row_shift, mut col_shift) = (row_target, col_target);
                    loop {
                        (row_shift, col_shift) = go(row_shift, col_shift, op);
                        let shift = idx(row_shift, col_shift);
                        match data[shift] {
                            '#' => continue 'op,
                            '.' | '@' => {
                                data.copy_within((shift + 1)..=target, shift);
                                data[target] = '@';
                                data[idx(row, col)] = '.';
                                (row, col) = (row_target, col_target);
                                continue 'op;
                            }
                            '[' | ']' => (),
                            _ => panic!("invalid data {}", data[shift]),
                        };
                    }
                }
                '[' if op == '>' => {
                    let (mut row_shift, mut col_shift) = (row_target, col_target);
                    loop {
                        (row_shift, col_shift) = go(row_shift, col_shift, op);
                        let shift = idx(row_shift, col_shift);
                        match data[shift] {
                            '#' => continue 'op,
                            '.' | '@' => {
                                data.copy_within(target..shift, target + 1);
                                data[target] = '@';
                                data[idx(row, col)] = '.';
                                (row, col) = (row_target, col_target);
                                continue 'op;
                            }
                            '[' | ']' => (),
                            _ => panic!("invalid data {}", data[shift]),
                        };
                    }
                }
                '[' | ']' => {
                    cands.clear();
                    cands_next.clear();

                    cands.push((row_target, col_target));
                    cands.push(if data[target] == '[' {
                        (row_target, col_target + 1)
                    } else {
                        (row_target, col_target - 1)
                    });
                    all_cands.clear();
                    all_cands.extend(cands.iter());

                    let possible = 'possible: loop {
                        cands_next.clear();
                        for (row_cand, col_cand) in &cands {
                            let (row, col) = go(*row_cand, *col_cand, op);
                            let idx = idx(row, col);
                            match data[idx] {
                                '#' => break 'possible false,
                                '.' | '@' => (),
                                '[' => {
                                    debug_assert!(data[idx + 1] == ']');
                                    cands_next.push((row, col));
                                    cands_next.push((row, col + 1));
                                }
                                ']' => {
                                    debug_assert!(data[idx - 1] == '[');
                                    cands_next.push((row, col));
                                    cands_next.push((row, col - 1));
                                }
                                _ => panic!("invalid data {}", data[idx]),
                            };
                        }
                        cands_next = cands_next.into_iter().unique().collect::<Vec<_>>();
                        if cands_next.is_empty() {
                            break 'possible true;
                        }
                        std::mem::swap(&mut cands, &mut cands_next);
                        all_cands.extend(cands.iter());
                    };
                    if possible {
                        for (row_cand, col_cand) in all_cands.iter().rev() {
                            let (row, col) = go(*row_cand, *col_cand, op);
                            debug_assert!(data[idx(row, col)] != '#');
                            data[idx(row, col)] = data[idx(*row_cand, *col_cand)];
                            data[idx(*row_cand, *col_cand)] = '.';
                        }
                        data[target] = '@';
                        data[idx(row, col)] = '.';
                        (row, col) = (row_target, col_target);
                    }
                }
                '#' => (),
                _ => panic!("invalid data {}", data[target]),
            }
        }

        let gps = |row, col| 100 * row + col;
        let mut res = 0;
        for row in 0..h {
            for col in 0..w {
                if data[idx(row, col)] == '[' {
                    res += gps(row, col);
                }
            }
        }
        Ok(Solution::U64(res as u64))
    }
}

pub fn sol(part: u8, input: &'static str) -> Solution {
    solve(part, input).expect("solve test")
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r"##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

    const SAMPLE_LIGHT: &str = r"#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^";

    #[test]
    fn part_1_sample() {
        assert_eq!(sol(1, SAMPLE), Solution::U64(10092));
    }

    #[test]
    fn part_1() {
        assert_eq!(sol(1, ""), Solution::U64(1516281));
    }

    #[test]
    fn part_2_sample_light() {
        assert_eq!(sol(2, SAMPLE_LIGHT), Solution::U64(618));
    }

    #[test]
    fn part_2_sample_big() {
        assert_eq!(sol(2, SAMPLE), Solution::U64(9021));
    }

    #[test]
    fn part_2() {
        assert_eq!(sol(2, ""), Solution::U64(1527969));
    }
}
