use hayatlib::matrix::transpose::Transpose;
use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        mut a: [[u32; w]; h],
    }
    a.transpose();
    for a in a {
        println!("{}", a.iter().join(" "));
    }
}
