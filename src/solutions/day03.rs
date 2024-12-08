use crate::Solution;
use anyhow::{Error, Result};
use itertools::Itertools;

fn valid(x: &str) -> Option<u64> {
    let n = x.chars().count();
    if n < 1 || n > 3 || x.chars().any(|x| !x.is_ascii_digit()) {
        None
    } else {
        Some(x.parse::<u64>().expect("inconsistency"))
    }
}

pub fn solve(part: u8, input: &'static str) -> Result<Solution, Error> {
    #![allow(unused)]
    let input = if input.len() > 0 {
        input
    } else {
        include_str!("../../data/03.input")
    };

    if part == 1 {
        let mut res = 0;
        for line in input.lines() {
            'tok: for tok in line.split("mul(") {
                if let Some(expr) = tok.split(")").next() {
                    if expr.split(",").count() != 2 {
                        continue 'tok;
                    }
                    let (num1, num2) = expr.split(",").collect_tuple().unwrap();
                    if let (Some(num1), Some(num2)) = (valid(num1), valid(num2)) {
                        res += num1 * num2;
                    }
                }
            }
        }
        Ok(Solution::U64(res))
    } else {
        let mut res = 0;
        let mut state_do = true;
        for line in input.lines() {
            for tok in line.split("mul(") {
                if state_do {
                    if let Some(expr) = tok.split(")").next() {
                        if expr.split(",").count() == 2 {
                            let (num1, num2) = expr.split(",").collect_tuple().unwrap();
                            if let (Some(num1), Some(num2)) = (valid(num1), valid(num2)) {
                                res += num1 * num2;
                            }
                        }
                    }
                }
                state_do = match (tok.find("do()"), tok.find("don't()")) {
                    (Some(idx_do), Some(idx_dont)) => idx_dont > idx_do,
                    (None, Some(..)) => false,
                    (Some(..), None) => true,
                    (None, None) => state_do,
                };
            }
        }
        Ok(Solution::U64(res))
    }
}

pub fn sol(part: u8, input: &'static str) -> Solution {
    solve(part, input).expect("solve test")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_sample() {
        const SAMPLE: &str =
            r"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(sol(1, SAMPLE), Solution::U64(161));
    }

    #[test]
    fn part_1() {
        assert_eq!(sol(1, ""), Solution::U64(165225049));
    }

    #[test]
    fn part_2_sample() {
        const SAMPLE: &str =
            r"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(sol(2, SAMPLE), Solution::U64(48));
    }

    #[test]
    fn part_2() {
        assert_eq!(sol(2, ""), Solution::U64(108830766));
    }
}
