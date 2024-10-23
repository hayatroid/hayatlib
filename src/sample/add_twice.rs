use super::add::add;

pub fn add_twice(a: u64, b: u64, c: u64) -> u64 {
    add(add(a, b), c)
}
