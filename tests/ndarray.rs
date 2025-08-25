//! Column-major indexing

use cart_lin::lin_to_cart;
use ndarray::{Array2, Array3, Array4, Axis};
use std::ptr::addr_of;

#[test]
fn test_lin_to_cart_2d() {
    let m = Array2::<f64>::zeros((5, 10));
    let nrows = m.nrows();
    let ncols = m.ncols();

    /*
    Convert a linear to a cartesian index and test that neighboring indices really are neighboring in memory
    by comparing the memory address
     */
    let indices: Vec<usize> = (0..nrows * ncols).into_iter().collect();
    for (win, (cartidx, _)) in indices.as_slice().windows(2).zip(m.indexed_iter()) {
        let c0 = lin_to_cart(win[0], &[nrows, ncols]).unwrap();
        let c1 = lin_to_cart(win[1], &[nrows, ncols]).unwrap();

        assert_eq!((c0[0], c0[1]), cartidx);

        let r0 = &m[(c0[0], c0[1])];
        let r1 = &m[(c1[0], c1[1])];

        let offset = addr_of!(r1) as usize - addr_of!(r0) as usize;
        assert_eq!(offset, std::mem::size_of::<f64>());
    }
}

#[test]
fn test_lin_to_cart_3d() {
    let m = Array3::<f64>::zeros((7, 5, 3));
    let n0 = m.len_of(Axis(0));
    let n1 = m.len_of(Axis(1));
    let n2 = m.len_of(Axis(2));

    /*
    Convert a linear to a cartesian index and test that neighboring indices really are neighboring in memory
    by comparing the memory address
     */
    let indices: Vec<usize> = (0..n0 * n1 * n2).into_iter().collect();
    for (win, (cartidx, _)) in indices.as_slice().windows(2).zip(m.indexed_iter()) {
        let c0 = lin_to_cart(win[0], &[n0, n1, n2]).unwrap();
        let c1 = lin_to_cart(win[1], &[n0, n1, n2]).unwrap();

        assert_eq!((c0[0], c0[1], c0[2]), cartidx);

        let r0 = &m[(c0[0], c0[1], c0[2])];
        let r1 = &m[(c1[0], c1[1], c0[2])];

        let offset = addr_of!(r1) as usize - addr_of!(r0) as usize;
        assert_eq!(offset, 4 * std::mem::size_of::<f64>());
    }
}

#[test]
fn test_lin_to_cart_4d() {
    let m = Array4::<f64>::zeros((7, 5, 3, 2));
    let n0 = m.len_of(Axis(0));
    let n1 = m.len_of(Axis(1));
    let n2 = m.len_of(Axis(2));
    let n3 = m.len_of(Axis(3));

    /*
    Convert a linear to a cartesian index and test that neighboring indices really are neighboring in memory
    by comparing the memory address
     */
    let indices: Vec<usize> = (0..n0 * n1 * n2 * n3).into_iter().collect();
    for (win, (cartidx, _)) in indices.as_slice().windows(2).zip(m.indexed_iter()) {
        let c0 = lin_to_cart(win[0], &[n0, n1, n2, n3]).unwrap();
        let c1 = lin_to_cart(win[1], &[n0, n1, n2, n3]).unwrap();

        assert_eq!((c0[0], c0[1], c0[2], c0[3]), cartidx);

        let r0 = &m[(c0[0], c0[1], c0[2], c0[3])];
        let r1 = &m[(c1[0], c1[1], c0[2], c0[3])];

        let offset = addr_of!(r1) as usize - addr_of!(r0) as usize;
        assert_eq!(offset, 5 * std::mem::size_of::<f64>());
    }
}

#[test]
fn test_cart_to_lin_2d() {
    let m = Array2::<f64>::zeros((5, 10));
    let n0 = m.len_of(Axis(0));
    let n1 = m.len_of(Axis(1));

    for (linidx, (cartidx, _)) in m.indexed_iter().enumerate() {
        let cartidx_calc = lin_to_cart(linidx, &[n0, n1]).unwrap();
        assert_eq!(cartidx.0, cartidx_calc[0]);
        assert_eq!(cartidx.1, cartidx_calc[1]);
    }
}

#[test]
fn test_cart_to_lin_3d() {
    let m = Array3::<f64>::zeros((5, 10, 37));
    let n0 = m.len_of(Axis(0));
    let n1 = m.len_of(Axis(1));
    let n2 = m.len_of(Axis(2));

    for (linidx, (cartidx, _)) in m.indexed_iter().enumerate() {
        let cartidx_calc = lin_to_cart(linidx, &[n0, n1, n2]).unwrap();
        assert_eq!(cartidx.0, cartidx_calc[0]);
        assert_eq!(cartidx.1, cartidx_calc[1]);
        assert_eq!(cartidx.2, cartidx_calc[2]);
    }
}

#[test]
fn test_cart_to_lin_4d() {
    let m = Array4::<f64>::zeros((5, 10, 7, 2));
    let n0 = m.len_of(Axis(0));
    let n1 = m.len_of(Axis(1));
    let n2 = m.len_of(Axis(2));
    let n3 = m.len_of(Axis(3));

    for (linidx, (cartidx, _)) in m.indexed_iter().enumerate() {
        let cartidx_calc = lin_to_cart(linidx, &[n0, n1, n2, n3]).unwrap();
        assert_eq!(cartidx.0, cartidx_calc[0]);
        assert_eq!(cartidx.1, cartidx_calc[1]);
        assert_eq!(cartidx.2, cartidx_calc[2]);
        assert_eq!(cartidx.3, cartidx_calc[3]);
    }
}
