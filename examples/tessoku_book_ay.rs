use hayatlib::io::query::Query;
use proconio::input;

fn main() {
    input! {
        q: usize,
        queries: [Query; q],
    }
    let mut stack = vec![];
    for query in queries {
        match query {
            Query::Q1(x) => stack.push(x),
            Query::Q2 => println!("{}", stack.last().unwrap()),
            Query::Q3 => {
                stack.pop();
            }
        }
    }
}
