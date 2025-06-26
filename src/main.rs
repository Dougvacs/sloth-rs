mod linalg;
mod mem;
use linalg::Vector;

use crate::linalg::Matrix;

fn main() {
    let v = Vector::<2>::new(&[1.0, 0.0]);
    let m = Matrix::<2, 2>::new(&[
        2.0, 0.0,
        0.0, 2.0
    ]);
    println!("{:?}", m * v)
}