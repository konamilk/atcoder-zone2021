use proconio::input;
#[allow(unused_imports)]
use proconio::source::auto::AutoSource;
#[allow(unused_imports)]
use proconio::marker::{Chars, Bytes};
#[allow(unused_imports)]
use num::integer::{sqrt, gcd, lcm};
#[allow(unused_imports)]
use std::cmp::{max, min, Reverse};
use smallvec::alloc::collections::VecDeque;

fn main() {
    let source = AutoSource::from("ozRnonnoe");
    input!{
        // from source,
        s: Chars
    };

    let mut t = VecDeque::new();
    let mut flg = true;

    for si in s{
        if si == 'R' {
            flg = !flg;
            continue
        }

        if flg{
            if t.len() == 0 {
                t.push_back(si)
            }
            else if t[t.len() - 1] != si {
                t.push_back(si)
            }
            else {
                t.pop_back();
            }
        }
        else {
            if t.len() == 0 {
                t.push_front(si)
            }
            else if t[0] != si {
                t.push_front(si)
            }
            else {
                t.pop_front();
            }
        }
    }

    if t.len() == 0{
        return
    }

    let mut ans;

    if flg {
        ans = t.into_iter().collect::<Vec<char>>();
    }
    else {
        ans = t.into_iter().rev().collect::<Vec<char>>();
    }

    println!("{}", ans.iter().collect::<String>());

}
