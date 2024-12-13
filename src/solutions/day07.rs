use crate::*;
use anyhow::{Context, Error, Result};
use rustc_hash::FxHashSet;

pub fn solve(part: u8, input: &'static str) -> Result<Solution, Error> {
    #![allow(unused)]
    let input = if input.len() > 0 {
        input
    } else {
        include_str!("../../data/07.input")
    };

    let mut res = 0;
    let mut cands = Vec::new();
    let mut cands_next = Vec::new();

    let mut lines = input.lines();
    for line in lines.by_ref() {
        if line.is_empty() {
            break;
        }
        let (result, equation) = line.split_once(':').context("no colon")?;
        let result = result.parse::<u64>().context("no pipe")?;
        cands.clear();

        for num in equation.split_whitespace() {
            let v = num.parse::<u64>().unwrap();
            cands_next.clear();
            if cands.is_empty() {
                cands_next.push(v);
            } else {
                for i in &cands {
                    if part == 1 {
                        cands_next.push(i + v);
                        cands_next.push(i * v);
                    } else {
                        cands_next.push(i + v);
                        cands_next.push(i * v);
                        let power = num.len() as u32;
                        cands_next.push(i * 10u64.pow(power) + v);
                    }
                }
            }
            std::mem::swap(&mut cands, &mut cands_next);
        }
        if cands.contains(&result) {
            res += result;
        }
    }

    Ok(Solution::U64(res))
}

pub fn sol(part: u8, input: &'static str) -> Solution {
    solve(part, input).expect("solve test")
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn part_1_sample() {
        assert_eq!(sol(1, SAMPLE), Solution::U64(3749));
    }

    #[test]
    fn part_1() {
        assert_eq!(sol(1, ""), Solution::U64(4555081946288));
    }

    #[test]
    fn part_2_sample() {
        assert_eq!(sol(2, SAMPLE), Solution::U64(11387));
    }

    #[test]
    fn part_2() {
        assert_eq!(sol(2, ""), Solution::U64(227921760109726));
    }
}
