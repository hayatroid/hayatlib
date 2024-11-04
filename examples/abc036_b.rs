use hayatlib::matrix::rotate::Rotate;
use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        mut s: [Chars; n],
    }
    s.rotate_cw();
    for s in s {
        println!("{}", s.iter().join(""));
    }
}
