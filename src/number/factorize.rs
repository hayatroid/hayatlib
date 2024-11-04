pub fn factorize(n: u64) -> Vec<u64> {
    assert!(n != 0);
    let mut res = vec![];
    let mut tmp = n;
    for i in (2..).take_while(|&i| i * i <= n) {
        while tmp % i == 0 {
            res.push(i);
            tmp /= i;
        }
    }
    if tmp > 1 {
        res.push(tmp);
    }
    res
}
