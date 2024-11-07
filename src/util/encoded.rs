pub trait Encoded<T> {
    fn encoded(&self) -> Vec<(T, usize)>;
}

impl<T: Clone + Eq> Encoded<T> for Vec<T> {
    fn encoded(&self) -> Vec<(T, usize)> {
        let mut z = self.iter().map(|a| (a.clone(), 1)).collect::<Vec<_>>();
        z.dedup_by(|a, b| (a.0 == b.0).then(|| b.1 += a.1).is_some());
        z
    }
}
