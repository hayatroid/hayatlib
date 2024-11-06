use hayatlib::util::compress::Compress;
use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [u32; n],
    }
    a.compress();
    for a in a {
        println!("{}", a);
    }
}
