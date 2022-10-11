mod shape;
mod tensor;

pub use shape::Shape;
pub use tensor::*;


pub fn test() {
    let t1 = tensor::full(&[3, 3, 3], 'h');
    let t3 = tensor::range(&[3, 3, 3]);
    let t2 = t3.get_sub(&[2]).unwrap();

    println!("{:?}", t3);
    println!("{:?}", t2);

}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
    }
}
