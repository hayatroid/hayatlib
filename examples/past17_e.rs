use hayatlib::util::encoded::Encoded;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }
    let t = s.encoded();
    for (c, l) in t {
        println!("{} {}", c, l);
    }
}
