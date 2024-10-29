fn modpow(mut x: u64, mut n: u64, modulus: u64) -> u64 {
    let mut res = 1;
    while n > 0 {
        if n & 1 == 1 {
            res = (res as u128 * x as u128 % modulus as u128) as u64;
        }
        x = (x as u128 * x as u128 % modulus as u128) as u64;
        n >>= 1;
    }
    res
}

pub fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }
    let s = (n - 1).trailing_zeros();
    let d = (n - 1) >> s;
    let is_witness = |a: u64| -> bool {
        a != 0 && modpow(a, d, n) != 1 && (0..s).all(|i| modpow(a, d << i, n) != n - 1)
    };
    let a = [2, 325, 9375, 28178, 450775, 9780504, 1795265022];
    a.iter().all(|&a| !is_witness(a % n))
}
