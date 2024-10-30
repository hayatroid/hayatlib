use std::io::{read_to_string, stdin};

use hayatlib::math::factorize_fast::factorize;

fn main() {
    let stdin = read_to_string(stdin()).unwrap();
    let mut stdin = stdin.split_whitespace();

    let q = stdin.next().unwrap().parse::<usize>().unwrap();
    let a = stdin
        .take(q)
        .map(|n| n.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    for a in a {
        let res = factorize(a);
        print!("{}", res.len());
        for res in res {
            print!(" {}", res);
        }
        println!();
    }
}
