use crate::Solution;

pub fn safe(levels: &[i8], skip: Option<usize>) -> bool {
    let mut prev = None;
    let mut increasing = None;
    for (idx, num) in levels.iter().enumerate() {
        if let Some(skip) = skip {
            if skip == idx {
                continue;
            }
        }
        if let Some(prev) = prev {
            let cur_increasing = num >= prev;
            match increasing {
                None => increasing = Some(cur_increasing),
                Some(increasing) => {
                    if cur_increasing != increasing {
                        return false;
                    }
                }
            }
            let delta = (num - prev).abs();
            if delta < 1 || delta > 3 {
                return false;
            }
        }
        prev = Some(num);
    }
    true
}

pub fn solve(part: u8, input: &'static str) -> Solution {
    #![allow(unused)]
    let input = if input.len() > 0 {
        input
    } else {
        include_str!("../../data/02.input")
    };

    let mut v = Vec::new();
    if part == 1 {
        let mut n_safe = 0;
        'line: for line in input.lines() {
            v.clear();
            for num in line.split_whitespace().map(|x| x.parse::<i8>().unwrap()) {
                v.push(num);
            }
            n_safe += safe(&v, None) as u64;
        }
        Solution::U64(n_safe)
    } else {
        let mut n_safe = 0;
        'line: for line in input.lines() {
            v.clear();
            for num in line.split_whitespace().map(|x| x.parse::<i8>().unwrap()) {
                v.push(num);
            }
            if safe(&v, None) {
                n_safe += 1;
            } else {
                let n = v.len();
                for skip in 0..n {
                    if safe(&v, Some(skip)) {
                        n_safe += 1;
                        continue 'line;
                    }
                }
            }
        }
        Solution::U64(n_safe)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn part_1_sample() {
        assert_eq!(solve(1, SAMPLE), Solution::U64(2));
    }

    #[test]
    fn part_1() {
        assert_eq!(solve(1, ""), Solution::U64(598));
    }

    #[test]
    fn part_2_sample() {
        assert_eq!(solve(2, SAMPLE), Solution::U64(4));
    }

    #[test]
    fn part_2() {
        assert_eq!(solve(2, ""), Solution::U64(634));
    }
}
