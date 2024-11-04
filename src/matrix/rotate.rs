use super::transpose::Transpose;

pub trait Rotate: Transpose {
    fn rotate_ccw(&mut self);
    fn rotate_cw(&mut self);
}

impl<T: Clone> Rotate for Vec<Vec<T>> {
    fn rotate_ccw(&mut self) {
        self.transpose();
        self.reverse();
    }
    fn rotate_cw(&mut self) {
        self.reverse();
        self.transpose();
    }
}
