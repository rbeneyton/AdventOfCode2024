use crate::Solution;
use itertools::Itertools;

fn xmas(x: &[char]) -> usize {
    x.iter()
        .tuple_windows()
        .filter(|(a, b, c, d)| **a == 'X' && **b == 'M' && **c == 'A' && **d == 'S')
        .count()
        + x.iter()
            .rev()
            .tuple_windows()
            .filter(|(a, b, c, d)| **a == 'X' && **b == 'M' && **c == 'A' && **d == 'S')
            .count()
}
#[test]
fn xmas_test() {
    let xmas_str = |x: &str| xmas(&x.chars().clone().collect::<Vec<char>>());
    assert_eq!(xmas_str(""), 0);
    assert_eq!(xmas_str("X"), 0);
    assert_eq!(xmas_str("XMAS"), 1);
    assert_eq!(xmas_str("XXXXXXMAS"), 1);
    assert_eq!(xmas_str("XMASSSSSS"), 1);
    assert_eq!(xmas_str("SAMX"), 1);
    assert_eq!(xmas_str("XXXX"), 0);
    assert_eq!(xmas_str("XMASXMAS"), 2);
    assert_eq!(xmas_str("XMASAMX"), 2);
}

pub fn solve(part: u8, input: &'static str) -> Solution {
    #![allow(unused)]
    let input = if input.len() > 0 {
        input
    } else {
        include_str!("../../data/04.input")
    };

    if part == 1 {
        // prepare
        let mut res = 0;
        let w = input.lines().next().unwrap().len();
        let h = input.lines().count();
        let mut grid = Vec::new();
        grid.resize((h * w) as usize, ' ');
        let idx = |row: usize, col: usize| (row.rem_euclid(h) * w + col.rem_euclid(w)) as usize;

        // load
        let mut v = Vec::new();
        for (row, line) in input.lines().enumerate() {
            for (col, c) in line.chars().enumerate() {
                grid[idx(row, col)] = c;
            }
        }

        // lines: by row
        for row in 0..h {
            v.clear();
            v.extend((0..w).map(|col| grid[idx(row, col)]));
            res += xmas(&v);
        }
        // rows: by col
        for col in 0..w {
            v.clear();
            v.extend((0..h).map(|row| grid[idx(row, col)]));
            res += xmas(&v);
        }

        // diags
        let m = 2 * std::cmp::max(w, h) as isize;
        let h_isize = h as isize;
        let w_isize = w as isize;
        for off in -(m + 1)..m {
            v.clear();
            for i in 0..m {
                let row = off + i;
                let col = i;
                if row >= 0 && row < h_isize && col < w_isize {
                    v.push(grid[idx(row as usize, col as usize)]);
                }
            }
            res += xmas(&v);

            v.clear();
            for i in 0..m {
                let row = off - i;
                let col = i;
                if row >= 0 && row < h_isize && col < w_isize {
                    v.push(grid[idx(row as usize, col as usize)]);
                }
            }
            res += xmas(&v);
        }

        Solution::USIZE(res)
    } else {
        // prepare
        let mut res = 0;
        let w = input.lines().next().unwrap().len();
        let h = input.lines().count();
        let mut grid = Vec::new();
        grid.resize((h * w) as usize, ' ');
        let idx = |row: usize, col: usize| (row.rem_euclid(h) * w + col.rem_euclid(w)) as usize;

        // load
        for (row, line) in input.lines().enumerate() {
            for (col, c) in line.chars().enumerate() {
                grid[idx(row, col)] = c;
            }
        }

        let xmas = |row, col| {
            if row > 0 && row < w - 1 && col > 0 && col < h - 1 {
                grid[idx(row, col)] == 'A'
                    && ((grid[idx(row - 1, col - 1)] == 'M' && grid[idx(row + 1, col + 1)] == 'S')
                        || (grid[idx(row - 1, col - 1)] == 'S'
                            && grid[idx(row + 1, col + 1)] == 'M'))
                    && ((grid[idx(row + 1, col - 1)] == 'M' && grid[idx(row - 1, col + 1)] == 'S')
                        || (grid[idx(row + 1, col - 1)] == 'S'
                            && grid[idx(row - 1, col + 1)] == 'M'))
            } else {
                false
            }
        };

        for row in 0..h {
            for col in 0..w {
                res += xmas(row, col) as usize;
            }
        }
        Solution::USIZE(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn part_1_sample() {
        assert_eq!(solve(1, SAMPLE), Solution::USIZE(18));
    }

    #[test]
    fn part_1() {
        assert_eq!(solve(1, ""), Solution::USIZE(2654));
    }

    #[test]
    fn part_2_sample() {
        assert_eq!(solve(2, SAMPLE), Solution::USIZE(9));
    }

    #[test]
    fn part_2() {
        assert_eq!(solve(2, ""), Solution::USIZE(1990));
    }
}
