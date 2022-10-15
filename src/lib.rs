mod shape;
mod tensor;

pub fn test() {
    let t1 = tensor::full(&[3, 3, 3], 'h');
    let t3 = tensor::range(&[3, 3, 3]);
    let t2 = t3.get_sub(&[2]).unwrap();
    let t4 = tensor::from_flat_vec(&[2, 2], vec![1, 2, 3, 3]).unwrap();

    println!("{:?}", t3);
    println!("{:?}", t2);
    println!("{:?}", t1);
    println!("{:?}", t4);
    println!("{:?}", t4.map(|x| x*2));

    let a = tensor::range(&[3, 2]);
    let b = tensor::range(&[2, 3]).map(|x| x * 2);
    let c = tensor::mul(&a, &b).unwrap();

    println!("{:?}", a);
    println!("{:?}", b);
    println!("{:?}", c);
    println!("{:?}", tensor::sum(&tensor::range(&[2, 2])));


    let identity = tensor::identity2(3);
    let mult = tensor::from_flat_vec(&[3, 1], vec![
                                     1.0,
                                     2.0,
                                     3.0,
    ]).unwrap();
    let output = tensor::matmul(&identity, &mult).unwrap();
    println!("{:?}", identity);
    println!("{:?}", mult);
    println!("{:?}", output);
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
    }
}
