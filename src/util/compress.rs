use std::fmt::Debug;

pub trait Compress {
    fn compress(&mut self);
}

impl<T> Compress for Vec<T>
where
    T: Clone + Ord + TryFrom<usize>,
    <T as TryFrom<usize>>::Error: Debug,
{
    fn compress(&mut self) {
        let mut z = self.clone();
        z.sort();
        z.dedup();
        *self = self
            .iter()
            .map(|a| z.binary_search(&a.clone()).unwrap().try_into().unwrap())
            .collect()
    }
}
