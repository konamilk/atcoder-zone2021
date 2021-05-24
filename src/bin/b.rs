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
    let source = AutoSource::from("5 896 483
    228 59
    529 310
    339 60
    78 266
    659 391");
    input!{
        // from source,
        n: usize,
        d: f64,
        h: f64,
        dh : [(f64, f64); n]
    };

    let mut ans = 0.0;

    for (di ,hi) in dh{
        let x = (d * hi - h * di)/ (d - di);
        // println!("{}", x);

        if x > ans {
            ans = x;
        }
    }

    println!("{}", ans);
}
