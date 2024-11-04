use super::transpose::transpose;

pub fn rotate_ccw<T: Clone>(v: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    let mut res = transpose(v);
    res.reverse();
    res
}

pub fn rotate_cw<T: Clone>(v: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    let mut v = v.clone();
    v.reverse();
    transpose(&v)
}
