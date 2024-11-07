use hayatlib::util::compressed::Compressed;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u32; n],
    }
    let b = a.compressed();
    for b in b {
        println!("{}", b);
    }
}
