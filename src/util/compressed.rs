pub trait Compressed {
    fn compressed(&self) -> Vec<usize>;
}

impl<T: Clone + Ord> Compressed for Vec<T> {
    fn compressed(&self) -> Vec<usize> {
        let mut z = self.clone();
        z.sort();
        z.dedup();
        self.iter()
            .map(|a| z.binary_search(&a.clone()).unwrap())
            .collect()
    }
}
