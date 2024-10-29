use std::io::{stdin, BufRead};

use hayatlib::math::is_prime::is_prime;

fn main() {
    let stdin = stdin();
    let mut stdin = stdin
        .lock()
        .lines()
        .map(Result::unwrap)
        .flat_map(|s| s.split_whitespace().map(str::to_string).collect::<Vec<_>>());
    let n = stdin.next().unwrap().parse::<usize>().unwrap();
    let a = stdin
        .take(n)
        .map(|a| a.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    println!("{}", a.iter().filter(|&&a| is_prime(a)).count());
}
