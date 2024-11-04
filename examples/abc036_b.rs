use hayatlib::matrix::rotate::rotate_cw;
use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: [Chars; n],
    }
    let ans = rotate_cw(&s);
    for ans in ans {
        println!("{}", ans.iter().join(""));
    }
}
