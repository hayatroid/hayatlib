use std::io::{read_to_string, stdin};

use hayatlib::number::is_prime_fast::is_prime;

fn main() {
    let stdin = read_to_string(stdin()).unwrap();
    let mut stdin = stdin.split_whitespace();

    let q = stdin.next().unwrap().parse::<usize>().unwrap();
    let n = stdin
        .take(q)
        .map(|n| n.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    for n in n {
        if is_prime(n) {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
