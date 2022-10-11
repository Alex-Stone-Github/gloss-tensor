use crate::shape::*;

#[derive(Debug)]
pub struct Tensor<T: Clone> {
    shape: Vec<usize>,
    elements: Vec<T>
}
impl<T: Clone> Tensor<T> {
    pub fn shape(&self) -> Shape {
        self.shape.as_slice()
    }
    pub fn rank(&self) -> usize {
        self.shape.len()
    }
    pub fn size(&self) -> usize {
        compute_size_of_shape(self.shape.as_slice())
    }
    pub fn get_value(&self, index: Shape) -> Result<T, &'static str> {
        let i = compute_index(self.shape.as_slice(), index)?;
        Ok(self.elements[i].clone())
    }
    pub fn set_value(&mut self, index: Shape, value: T) -> Result<(), &'static str> {
        let i = compute_index(self.shape.as_slice(), index)?;
        self.elements[i] = value;
        Ok(())
    }
    pub fn get_sub(&self, index: Shape) -> Result<Self, &'static str> {
        let ind = compute_index(self.shape.as_slice(), index)?;
        // (3, 3, 3) - (2, 2) = (3)
        let tmp = self.shape.len() - index.len();
        let new_shape = &self.shape[(self.shape.len()-tmp)..];

        let new_size = compute_size_of_shape(new_shape);
        let mut new_elements = Vec::<T>::with_capacity(new_size);
        // cruddy memcpy
        println!("{}", ind);
        for i in 0..new_size {
            new_elements.push(self.elements[ind + i].clone());
        }
        Ok(Self {
            shape: new_shape.iter().map(|x| *x).collect(),
            elements: new_elements
        })
    }
    pub fn reshape(&mut self, shape: Shape) -> Result<(), &'static str> {
        if compute_size_of_shape(self.shape.as_slice()) != compute_size_of_shape(shape) {
            return Err("New shape must have same size");
        }
        self.shape = shape.iter().map(|x| *x).collect();
        Ok(())
    }
    pub fn view(&self, shape: Shape) -> Result<Self, &'static str> {
        if compute_size_of_shape(self.shape.as_slice()) != compute_size_of_shape(shape) {
            return Err("New shape must have same size");
        }
        Ok(Self {
            shape: shape.iter().map(|x| *x).collect(),
            elements: self.elements.clone()
        })
    }
}
pub fn full<T: Clone>(shape: Shape, value: T) -> Tensor<T> {
    let length = compute_size_of_shape(shape);
    let mut elements = Vec::<T>::with_capacity(length);
    // crudy memcpy
    for _ in 0..length {elements.push(value.clone());}
    //elements.fill(value);
    Tensor {
        shape: shape.iter().map(|x| *x).collect(),
        elements
    }
}
pub fn range(shape: Shape) -> Tensor<i32> {
    let length = compute_size_of_shape(shape);
    let elements = (0..length).map(|x| x as i32).collect();
    Tensor {
        shape: shape.iter().map(|x| *x).collect(),
        elements
    }
}



