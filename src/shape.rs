pub type Shape<'a> = &'a [usize];

pub fn compute_size_of_shape(shape: Shape) -> usize {
    let mut size: usize = 1;
    for v in shape.iter() {
        size *= v;
    }
    size
}
pub fn compute_index(shape: Shape, index: Shape) -> Result<usize, &'static str> {
    // we need to make sure i is not longer than s
    if index.len() > shape.len() {
        return Err("Index shape is longer");
    }
    // we need to make sure that every element in s is > every element in i
    for i in 0..index.len() {
        if index[i] > shape[i] {
            return Err("Elements in index cannot be larger");
        }
    }
    let mut tmp: usize = 0;
    for (i, val) in index.iter().enumerate() {
        if i == shape.len() - 1 {
            tmp += val;
        }
        else {
            // get the total of the smaller shape
            let smaller_shape = &shape[i+1..];
            // multiply smaller shape by val and add to tmp
            tmp += val * compute_size_of_shape(smaller_shape);
        }
    }
    Ok(tmp)
}
