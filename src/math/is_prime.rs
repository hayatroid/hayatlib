pub fn is_prime(n: u64) -> bool {
    n >= 2 && (2..).take_while(|&i| i * i <= n).all(|i| n % i != 0)
}
