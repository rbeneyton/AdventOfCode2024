use crate::*;

pub fn solve(part: u8, input: &'static str) -> Solution {
    #![allow(unused)]
    let input = if input.len() > 0 {
        input
    } else {
        include_str!("../../data/01.input")
    };
    // FIXME proconio on str
    // durstart!(durations);
    // input! {
    //     v: [(u32, u32)],
    // }
    // dur!(durations, "input done");

    if part == 1 {
        let mut v = [Vec::new(), Vec::new()];
        for line in input.lines() {
            for (idx, num) in line.split_whitespace().enumerate() {
                let v = &mut v[idx];
                let num = num.parse::<i64>().unwrap();
                let pos = v.binary_search(&num).unwrap_or_else(|e| e);
                v.insert(pos, num);
            }
        }
        let n = v[0].len();
        debug_assert_eq!(n, v[1].len());

        let res = (0..n).map(|i| (v[0][i] - v[1][i]).abs()).sum();
        Solution::I64(res)
    } else {
        let mut h = [FxHashMap::default(), FxHashMap::default()];
        for line in input.lines() {
            for (idx, num) in line.split_whitespace().enumerate() {
                let h = &mut h[idx];
                let num = num.parse::<i64>().unwrap();
                h.entry(num).and_modify(|k| *k += 1).or_insert(1);
            }
        }

        let mut res = 0;
        for (k, v) in &h[0] {
            if let Some(c) = h[1].get(k) {
                res += k * *v * *c;
            }
        }
        Solution::I64(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r"3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn part_1_sample() {
        assert_eq!(solve(1, SAMPLE), Solution::I64(11));
    }

    #[test]
    fn part_1() {
        assert_eq!(solve(1, ""), Solution::I64(2166959));
    }

    #[test]
    fn part_2_sample() {
        assert_eq!(solve(2, SAMPLE), Solution::I64(31));
    }

    #[test]
    fn part_2() {
        assert_eq!(solve(2, ""), Solution::I64(23741109));
    }
}
