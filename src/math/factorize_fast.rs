use crate::math::is_prime_fast::is_prime;

fn gcd(x: u64, y: u64) -> u64 {
    if y == 0 {
        return x;
    }
    gcd(y, x % y)
}

fn factorize_inner(n: u64) -> Vec<u64> {
    assert!(n != 0);
    if n == 1 {
        return vec![];
    }
    if is_prime(n) {
        return vec![n];
    }
    let non_trivial_divisor_of = |n: u64| -> u64 {
        if n % 2 == 0 {
            return 2;
        }
        for step in 1.. {
            let f = |x: u64| -> u64 { ((x as u128 * x as u128 + step as u128) % n as u128) as u64 };
            let mut x = step;
            let mut y = f(x);
            let mut d = 1;
            while d == 1 {
                x = f(x);
                y = f(f(y));
                d = gcd(x.abs_diff(y), n);
            }
            if d != n {
                return d;
            }
        }
        unreachable!()
    };
    let d = non_trivial_divisor_of(n);
    let mut l = factorize_inner(d);
    let r = factorize_inner(n / d);
    l.extend(r);
    l
}

pub fn factorize(n: u64) -> Vec<u64> {
    let mut res = factorize_inner(n);
    res.sort();
    res
}
