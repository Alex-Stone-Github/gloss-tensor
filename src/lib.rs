mod shape;
use shape::{Shape, compute_size_of_shape};

pub trait Tensor<T> {
    fn shape(&self) -> Shape;
    fn rank(&self) -> usize;
    fn size(&self) -> usize;
    fn get_value(&self, index: Shape) -> &T;
    fn get_value_mut(&mut self, index: Shape) -> &mut T;
}

pub struct OwnedTensor<T> {
    shape: Vec<usize>,
    elements: Vec<T>
}
impl<T> Tensor<T> for OwnedTensor<T> {
    fn shape(&self) -> Shape {
        self.shape.as_slice()
    }
    fn rank(&self) -> usize {
        self.shape.len()
    }
    fn size(&self) -> usize {
        compute_size_of_shape(self.shape.as_slice())
    }
    fn get_value(&self, index: Shape) -> &T {
        &self.elements[0]
    }
    fn get_value_mut(&mut self, index: Shape) -> &mut T {
        &mut self.elements[0]
    }
}


pub fn test() {
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
    }
}
