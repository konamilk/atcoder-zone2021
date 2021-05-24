use proconio::input;
#[allow(unused_imports)]
use proconio::source::auto::AutoSource;
#[allow(unused_imports)]
use proconio::marker::{Chars, Bytes};
#[allow(unused_imports)]
use num::integer::{sqrt, gcd, lcm};
#[allow(unused_imports)]
use std::cmp::{max, min, Reverse};

fn main() {
    // let source = AutoSource::from("0");
    input!{
        // from source,
        n: usize,
        abcde: [(i64, i64, i64, i64, i64); n]
    };

    let mut dp = vec![vec![std::i64::MAX; 4];n+1];

    for i in 0..n {
        dp[i+1][w]
    }
}
