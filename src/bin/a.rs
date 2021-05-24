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
        s: String
    };

    let mut ans = 0;

    for i in 0..s.len()-3{
        let t = &s[i..i+4];
        // println!("{}", t);
        if t == String::from("ZONe") {
            ans += 1;
        }
    }

    println!("{}", ans);
}
