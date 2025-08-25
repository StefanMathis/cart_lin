//! Column-major indexing

use cart_lin::{cart_to_lin, lin_to_cart};
use nalgebra::{DMatrix, DVector, Dim, Matrix, Matrix2x3, RawStorage, Vector3};
use std::ptr::addr_of;

#[test]
fn test_lin_to_cart() {
    lin_to_cart_nalgebra(Matrix2x3::from_element(1.0));
    lin_to_cart_nalgebra(DMatrix::from_element(5, 10, 1.0));
    lin_to_cart_nalgebra(DMatrix::from_element(1, 10, 1.0));
    lin_to_cart_nalgebra(DMatrix::from_element(10, 1, 1.0));
    lin_to_cart_nalgebra(Vector3::from_element(1.0));
    lin_to_cart_nalgebra(DVector::from_element(10, 1.0));
}

fn lin_to_cart_nalgebra<R: Dim, C: Dim, S: RawStorage<f64, R, C>>(m: Matrix<f64, R, C, S>) {
    let nrows = m.nrows();
    let ncols = m.ncols();

    /*
    Convert a linear to a cartesian index and test that neighboring indices really are neighboring in memory
    by comparing the memory address
     */
    let indices: Vec<usize> = (0..nrows * ncols).into_iter().collect();
    for win in indices.as_slice().windows(2) {
        let c0 = lin_to_cart(win[0], &[nrows, ncols]).unwrap();
        let c1 = lin_to_cart(win[1], &[nrows, ncols]).unwrap();

        let r0 = &m[(c0[0], c0[1])];
        let r1 = &m[(c1[0], c1[1])];

        let offset = addr_of!(r1) as usize - addr_of!(r0) as usize;
        assert_eq!(offset, std::mem::size_of::<f64>());
    }
}

#[test]
fn test_cart_to_lin() {
    cart_to_lin_nalgebra(Matrix2x3::from_element(1.0));
    cart_to_lin_nalgebra(DMatrix::from_element(5, 10, 1.0));
    cart_to_lin_nalgebra(DMatrix::from_element(1, 10, 1.0));
    cart_to_lin_nalgebra(DMatrix::from_element(10, 1, 1.0));
    cart_to_lin_nalgebra(Vector3::from_element(1.0));
    cart_to_lin_nalgebra(DVector::from_element(10, 1.0));
}

fn cart_to_lin_nalgebra<R: Dim, C: Dim, S: RawStorage<f64, R, C>>(m: Matrix<f64, R, C, S>) {
    // nalgebra uses column-major order
    let mut indices = Vec::with_capacity(m.nrows() * m.ncols());
    for row in 0..m.nrows() {
        for col in 0..m.ncols() {
            indices.push((row, col));
        }
    }

    for win in indices.as_slice().windows(2) {
        let r0 = &m[win[0]];
        let r1 = &m[win[1]];

        let offset = addr_of!(r1) as usize - addr_of!(r0) as usize;
        assert_eq!(offset, std::mem::size_of::<f64>());

        let offset = cart_to_lin(&[win[1].0, win[1].1], &[m.nrows(), m.ncols()]).unwrap()
            - cart_to_lin(&[win[0].0, win[0].1], &[m.nrows(), m.ncols()]).unwrap();
        assert_eq!(offset, 1);
    }
}
