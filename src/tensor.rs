use crate::shape::*;

#[derive(Debug, Clone)]
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
    pub fn map(&self, rule: fn(T) -> T) -> Self {
        let mut new_tensor = self.clone();
        for i in 0..new_tensor.elements.len() {
            new_tensor.elements[i] = rule(new_tensor.elements[i].clone());
        }
        new_tensor
    }
}

pub use create::*;
pub use ops::*;

mod ops {
    use super::*;
    pub fn individual_binary_operation<T: Clone>(a: &Tensor<T>, b: &Tensor<T>, rule: fn(T, T) -> T)
        -> Result<Tensor<T>, &'static str> {
        if a.size() != b.size() {
            return Err("Size of a and b tensors must be the same");
        }
        let mut new_tensor = a.clone();
        for i in 0..new_tensor.elements.len() {
            new_tensor.elements[i] = rule(new_tensor.elements[i].clone(), b.elements[i].clone());
        }
        Ok(new_tensor)
    }
    pub fn add<T: Clone + std::ops::Add<Output=T>>(a: &Tensor<T>, b: &Tensor<T>)
        -> Result<Tensor<T>, &'static str> {
        Ok(individual_binary_operation(a, b, |x, y| x + y)?)
    }
    pub fn sub<T: Clone + std::ops::Sub<Output=T>>(a: &Tensor<T>, b: &Tensor<T>)
        -> Result<Tensor<T>, &'static str> {
        Ok(individual_binary_operation(a, b, |x, y| x - y)?)
    }
    pub fn mul<T: Clone + std::ops::Mul<Output=T>>(a: &Tensor<T>, b: &Tensor<T>)
        -> Result<Tensor<T>, &'static str> {
        Ok(individual_binary_operation(a, b, |x, y| x * y)?)
    }
    pub fn div<T: Clone + std::ops::Div<Output=T>>(a: &Tensor<T>, b: &Tensor<T>)
        -> Result<Tensor<T>, &'static str> {
        Ok(individual_binary_operation(a, b, |x, y| x / y)?)
    }
    pub fn sum<T: Clone + std::ops::Add<Output=T> + Default>(summed: &Tensor<T>) -> T {
        let mut summation = T::default(); // default used here
        for element in summed.elements.iter() {
            summation = summation + element.clone();
        }
        summation
    }
    pub fn dot<T: Clone + std::ops::Mul<Output=T> + std::ops::Add<Output=T> + Default>(a: &Tensor<T>,
                                                   b: &Tensor<T>) -> Result<T, &'static str> {
        Ok(sum(&(mul(a, b)?)))
    }
    pub fn matmul<T>(a: &Tensor<T>, b: &Tensor<T>) -> Result<Tensor<T>, &'static str>
        where T: Clone + Default + std::ops::Mul<Output=T>
        + std::ops::AddAssign,
    {
        if a.shape.len() != 2 && b.shape.len() != 2 {
            return Err("Cannot multiply matrices with dimensionality != 2");
        }
        if a.shape[1] != b.shape[0] {
            return Err("Matrix a columns must match matrix b rows");
        }
        /*
         * rows by columns
         * first row then column
         * a: 3*2 matrix
         * b: 2*3 matrix
         * c = 3 * 3
         */
        let mut new_tensor = full(&[a.shape[0], b.shape[1]], T::default());

        for i in 0..a.shape[0] {
            for j in 0..b.shape[1] {
                let mut value = T::default();
                for k in 0..b.shape[0] {
                    value += a.get_value(&[i, k]).unwrap() * b.get_value(&[k, j]).unwrap();
                }
                new_tensor.set_value(&[i, j], value).expect("Something went terribly wrong");
            }
        }
        Ok(new_tensor)
    }
    pub fn transpose<T: Clone + Default>(x: &Tensor<T>) -> Result<Tensor<T>, &'static str> {
        if x.shape.len() != 2 {
            return Err("To transpose a tensor the rank must be 2");
        }
        let mut new_tensor = create::full(&[x.shape[1], x.shape[0]], T::default());
        for i in 0..new_tensor.shape[0] {
            for j in 0..new_tensor.shape[1] {
                let _ = new_tensor.set_value(&[i, j], x.get_value(&[j, i]).expect("Something terribly
                    wrong has happened with the transpose function"));
            }
        }
        Ok(new_tensor)
    }
}

mod create {
    use super::*;
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
    pub fn from_flat_vec<T: Clone>(shape: Shape, elements: Vec<T>) -> Result<Tensor<T>, &'static str> {
        if elements.len() != compute_size_of_shape(shape) {
            return Err("Element count does not match size of shape")
        }
        Ok(Tensor {
            shape: shape.iter().map(|x| *x).collect(),
            elements
        })
    }
    pub fn identity2(count: usize) -> Tensor<f64> {
        let mut new_tensor = full(&[count, count], 0.0);
        for i in 0..count {
            new_tensor.set_value(&[i, i], 1.0).expect("Something went terribly wrong");
        }
        new_tensor
    }
}


