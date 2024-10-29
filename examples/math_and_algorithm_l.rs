use hayatlib::math::is_prime::is_prime;
use proconio::input;

fn main() {
    input! {
        n: u64,
    }
    if is_prime(n) {
        println!("Yes");
    } else {
        println!("No");
    }
}
