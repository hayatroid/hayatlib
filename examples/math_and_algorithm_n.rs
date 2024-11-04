use hayatlib::number::factorize::factorize;
use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: u64,
    }
    let mut res = factorize(n);
    println!("{}", res.iter().join(" "));
}
