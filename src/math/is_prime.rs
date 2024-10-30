pub fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }
    (2..).take_while(|&i| i * i <= n).all(|i| n % i != 0)
}
