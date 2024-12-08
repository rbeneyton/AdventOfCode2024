use crate::Solution;
use anyhow::{Context, Result};
use rustc_hash::{FxHashMap, FxHashSet};

pub fn solve(part: u8, input: &'static str) -> Result<Solution, anyhow::Error> {
    #![allow(unused)]
    let input = if input.len() > 0 {
        input
    } else {
        include_str!("../../data/05.input")
    };

    let mut rules = FxHashMap::default();

    let mut lines = input.lines();
    for line in lines.by_ref() {
        if line.is_empty() {
            break;
        }
        let (page, dep) = line.split_once('|').context("no pipe")?;
        let page = page.parse::<u64>().context("no pipe")?;
        let dep = dep.parse::<u64>().context("no pipe")?;
        rules
            .entry(page)
            .and_modify(|v: &mut Vec<u64>| {
                v.insert(v.binary_search(&dep).unwrap_or_else(|e| e), dep);
            })
            .or_insert(vec![dep]);
    }
    log::debug!(
        "rules: \n{}",
        itertools::join(
            rules.iter().map(|(k, v)| format!(
                "{k}:{}",
                itertools::join(v.iter().map(|xx| format!("{}", xx)), ",")
            )),
            "\n"
        )
    );

    if part == 1 {
        let mut res = 0;
        'line: for line in lines {
            let updates = line
                .split(',')
                .map(|x| x.parse::<u64>().expect("no pipe"))
                .collect::<Vec<_>>();
            log::debug!(
                "updates: {}",
                itertools::join(updates.iter().map(|x| format!("{}", x)), ",")
            );
            for (i, up) in updates.iter().enumerate() {
                let Some(rule) = rules.get(up) else {
                    continue;
                };
                for pre in updates.iter().take(i) {
                    if rule.binary_search(pre).is_ok() {
                        continue 'line;
                    }
                }
            }
            res += updates[updates.len() / 2];
        }

        Ok(Solution::U64(res))
    } else {
        let mut res = 0;
        'line: for line in lines {
            let updates = line
                .split(',')
                .map(|x| x.parse::<u64>().expect("no pipe"))
                .collect::<Vec<_>>();
            'valid: loop {
                for (i, up) in updates.iter().enumerate() {
                    let Some(rule) = rules.get(up) else {
                        continue;
                    };
                    for pre in updates.iter().take(i) {
                        if rule.binary_search(pre).is_ok() {
                            break 'valid;
                        }
                    }
                }
                continue 'line;
            }
            log::debug!(
                "updates: {}",
                itertools::join(updates.iter().map(|x| format!("{}", x)), ",")
            );
            let mut sorted = Vec::new();
            let mut sorted_h = FxHashSet::default();
            loop {
                let mut next = None;
                'i: for (i, up) in updates.iter().enumerate() {
                    if sorted_h.contains(&up) {
                        continue;
                    }
                    for (j, o) in updates.iter().enumerate() {
                        if (i == j) {
                            continue;
                        }
                        if sorted_h.contains(&o) {
                            continue;
                        }
                        if let Some(rule) = rules.get(o) {
                            if rule.binary_search(up).is_ok() {
                                continue 'i;
                            }
                        }
                    }
                    next = Some(up);
                    break;
                }
                let next = next.expect("no ordering allowed!");
                sorted.push(next);
                sorted_h.insert(next);
                log::debug!(
                    "sorted: {}",
                    itertools::join(sorted.iter().map(|x| format!("{}", x)), ",")
                );
                if sorted.len() == updates.len() {
                    break;
                }
            }
            log::debug!(
                "sorted: {}",
                itertools::join(sorted.iter().map(|x| format!("{}", x)), ",")
            );
            res += sorted[sorted.len() / 2];
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

    const SAMPLE: &str = r"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn part_1_sample() {
        assert_eq!(sol(1, SAMPLE), Solution::U64(143));
    }

    #[test]
    fn part_1() {
        assert_eq!(sol(1, ""), Solution::U64(4774));
    }

    #[test]
    fn part_2_sample() {
        assert_eq!(sol(2, SAMPLE), Solution::U64(123));
    }

    #[test]
    fn part_2() {
        assert_eq!(sol(2, ""), Solution::U64(6004));
    }
}
