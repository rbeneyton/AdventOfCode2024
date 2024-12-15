use crate::*;
use anyhow::{Context, Error, Result};
use rustc_hash::FxHashSet;

pub fn solve(part: u8, input: &'static str) -> Result<Solution, Error> {
    #![allow(unused)]
    let input = if input.len() > 0 {
        input
    } else {
        include_str!("../../data/08.input")
    };

    let w = input.lines().next().context("no 1st line")?.len();
    let data = input.chars().filter(|x| *x != '\n').collect::<Vec<_>>();
    let n = data.len();
    assert!(n % w == 0);
    let h = n / w;
    let (w, h) = (w as isize, h as isize);
    let idx = |row, col| (row * w + col) as usize;
    let valid = |row, col| row >= 0 && row < h && col >= 0 && col < w;

    let mut res = 0;
    let mut antennas = BTreeMap::default();

    for row in 0..h {
        for col in 0..w {
            let c = data[idx(row, col)];
            if c == '.' {
                continue;
            }
            antennas
                .entry(c)
                .and_modify(|k: &mut Vec<_>| k.push((row, col)))
                .or_insert(vec![(row, col)]);
        }
    }

    let mut cands = HashSet::new();
    for (c, v) in antennas {
        let n = v.len();
        for i in 0..n {
            let a = v[i];
            for j in (i + 1)..n {
                let b = v[j];
                let drow = b.0 - a.0;
                let dcol = b.1 - a.1;

                if part == 1 {
                    let (row, col) = (a.0 - drow, a.1 - dcol);
                    if valid(row, col) {
                        cands.insert((row, col));
                    }
                    let (row, col) = (b.0 + drow, b.1 + dcol);
                    if valid(row, col) {
                        cands.insert((row, col));
                    }
                }
                if part == 2 {
                    let spread = std::cmp::max(w, h) as isize;
                    let start = if part == 1 { 1 } else { -spread };
                    let end = if part == 1 { 2 } else { spread };
                    for scale in start..end {
                        let (row, col) = (a.0 + scale * drow, a.1 + scale * dcol);
                        if valid(row, col) {
                            cands.insert((row, col));
                        }
                    }
                }
            }
        }
    }

    Ok(Solution::U64(cands.len() as u64))
}

pub fn sol(part: u8, input: &'static str) -> Solution {
    solve(part, input).expect("solve test")
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r"............
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
............";

    #[test]
    fn part_1_sample() {
        assert_eq!(sol(1, SAMPLE), Solution::U64(14));
    }

    #[test]
    fn part_1() {
        assert_eq!(sol(1, ""), Solution::U64(271));
    }

    #[test]
    fn part_2_sample() {
        assert_eq!(sol(2, SAMPLE), Solution::U64(34));
    }

    #[test]
    fn part_2() {
        assert_eq!(sol(2, ""), Solution::U64(994));
    }
}
