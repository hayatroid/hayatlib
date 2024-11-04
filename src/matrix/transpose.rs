pub fn transpose<T: Clone>(v: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    if v.len() == 0 {
        return vec![]
    }
    (0..v[0].len())
        .map(|j| (0..v.len()).map(|i| v[i][j].clone()).collect())
        .collect()
}
