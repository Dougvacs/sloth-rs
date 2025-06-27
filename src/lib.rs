mod linalg;
mod mem;

#[cfg(test)]
mod linalg_tests {
    use std::f32::consts::PI;

    use crate::linalg::{Matrix, Vector};

    use super::*;

    #[test]
    fn matrix_vector() {
        let I = Matrix::<2, 2>::new(&[
            1.0, 0.0,
            0.0, 1.0
        ]);
        let R = |x: f32| { 
            return Matrix::<2, 2>::new(&[
                f32::cos(x), -f32::sin(x),
                f32::sin(x), f32::cos(x)
        ])};
        let y = Vector::<2>::new(&[0.0, 1.0]);
        let x = Vector::<2>::new(&[1.0, 0.0]);
        assert_eq!(I*y, y);
        assert!(4.0*I*y > y);
        println!("{:?}", R(PI/2.0) * y);
        assert!(R(PI/2.0) * y == x)
    }
}
