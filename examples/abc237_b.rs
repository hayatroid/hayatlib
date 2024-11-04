use hayatlib::matrix::transpose::transpose;
use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[u32; w]; h],
    }
    let ans = transpose(&a);
    for ans in ans {
        println!("{}", ans.iter().join(" "));
    }
}
