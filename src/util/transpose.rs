pub trait Transpose {
    fn transpose(&mut self);
}

impl<T: Clone> Transpose for Vec<Vec<T>> {
    fn transpose(&mut self) {
        if self.len() == 0 {
            return;
        }
        *self = (0..self[0].len())
            .map(|j| (0..self.len()).map(|i| self[i][j].clone()).collect())
            .collect()
    }
}
