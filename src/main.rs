mod linalg;
use linalg::Vector;

fn main() {
    let v = Vector::<3>::new(&[1.0, 1.0, 1.0]);
    println!("{:?}", v | v)
}