#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(dead_code)]

pub(crate) use itertools::Itertools;
pub(crate) use proconio::input;
pub(crate) use proconio::marker::*;
pub(crate) use rustc_hash::{FxHashMap, FxHashSet};
pub(crate) use std::{
    cmp,
    collections::*,
    fmt,
    io::{self, Write},
    mem::swap,
};

#[allow(unused_variables)]
const LOG_LVL: u8 = 0;

macro_rules! logstart {
    ($lvl:expr, $($arg:tt)+) => ({
        let lvl = $lvl;
        if lvl <= LOG_LVL {
            eprint!("L{}:L{}:{}",
                line!(),
                lvl,
                fmt::format(format_args!($($arg)+)));
        }
    })
}
pub(crate) use logstart;

macro_rules! logcont {
    ($lvl:expr, $($arg:tt)+) => ({
        let lvl = $lvl;
        if lvl <= LOG_LVL {
            eprint!("{}",
                fmt::format(format_args!($($arg)+)));
        }
    })
}
pub(crate) use logcont;

macro_rules! logstop {
    ($lvl:expr, $($arg:tt)+) => ({
        let lvl = $lvl;
        if lvl <= LOG_LVL {
            eprintln!("{}",
                fmt::format(format_args!($($arg)+)));
        }
    })
}
pub(crate) use logstop;

macro_rules! logln {
    ($lvl:expr, $($arg:tt)+) => ({
        let lvl = $lvl;
        if lvl <= LOG_LVL {
            eprintln!("L{}:L{}:{}",
                line!(),
                lvl,
                fmt::format(format_args!($($arg)+)));
            io::stderr().flush().unwrap();
        }
    })
}
pub(crate) use logln;

const DURATIONS_SZ: usize = 64;
pub(crate) struct Durations {
    v: [(&'static str, std::time::Instant); DURATIONS_SZ],
    idx: usize,
}
impl Durations {
    pub fn new() -> Self {
        Self {
            v: [("start", std::time::Instant::now()); DURATIONS_SZ],
            idx: 1,
        }
    }
    pub fn push(&mut self, label: &'static str) {
        debug_assert!(self.idx < DURATIONS_SZ);
        self.v[self.idx] = (label, std::time::Instant::now());
        self.idx += 1;
    }
}
impl Drop for Durations {
    fn drop(&mut self) {
        self.push("end");
        let startup = self.v[0].1;
        let mut prev = None;
        for idx in 1..(self.idx) {
            let (what, timestamp) = self.v[idx];
            let duration = timestamp.saturating_duration_since(startup);
            if let Some(prev) = prev {
                let duration_from_prev = timestamp.saturating_duration_since(prev);
                eprintln!(
                    "{:>10.6} ({:>10.6}) {}",
                    duration.as_secs_f64(),
                    duration_from_prev.as_secs_f64(),
                    what
                );
            } else {
                eprintln!("{:>10.6}  {:>10}  {}", duration.as_secs_f64(), "", what);
            }
            prev = Some(timestamp);
        }
    }
}

macro_rules! durstart {
    ($name:ident) => {
        #[cfg(feature = "durations")]
        let mut $name = Durations::new();
    };
}
pub(crate) use durstart;

macro_rules! dur {
    ($name:ident, $label:literal) => {
        #[cfg(feature = "durations")]
        $name.push($label);
    };
}
pub(crate) use dur;

pub fn get_stdin_line() -> String {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line.trim_end().to_string()
}

pub type Day = i8;

#[derive(Debug, PartialEq)]
pub enum Solution {
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    ISIZE(isize),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    USIZE(usize),
    Str(String),
}

use std::fmt::{Display, Formatter, Result};
impl Display for Solution {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Solution::I16(x) => x.fmt(f),
            Solution::I32(x) => x.fmt(f),
            Solution::I64(x) => x.fmt(f),
            Solution::I128(x) => x.fmt(f),
            Solution::ISIZE(x) => x.fmt(f),
            Solution::U16(x) => x.fmt(f),
            Solution::U32(x) => x.fmt(f),
            Solution::U64(x) => x.fmt(f),
            Solution::U128(x) => x.fmt(f),
            Solution::USIZE(x) => x.fmt(f),
            Solution::Str(x) => x.fmt(f),
        }
    }
}

pub mod load;
pub use load::get_data_server;

pub mod solutions;
pub use solutions::solve;
