use cart_lin::{cart_to_lin, lin_to_cart, lin_to_cart_dyn};

#[test]
fn test_cart_to_lin_1d() {
    assert_eq!(cart_to_lin(&[1], &[5]).unwrap(), 1);
    assert_eq!(cart_to_lin(&[4], &[5]).unwrap(), 4);
}

#[test]
fn test_cart_to_lin_2d() {
    {
        // 3 x 3 matrix with the following linear indexing order
        // [0 3 6]
        // [1 4 7]
        // [2 5 8]
        assert_eq!(cart_to_lin(&[0, 0], &[3, 3]).unwrap(), 0);
        assert_eq!(cart_to_lin(&[1, 0], &[3, 3]).unwrap(), 3);
        assert_eq!(cart_to_lin(&[2, 0], &[3, 3]).unwrap(), 6);
        assert_eq!(cart_to_lin(&[0, 1], &[3, 3]).unwrap(), 1);
        assert_eq!(cart_to_lin(&[1, 1], &[3, 3]).unwrap(), 4);
        assert_eq!(cart_to_lin(&[2, 1], &[3, 3]).unwrap(), 7);
        assert_eq!(cart_to_lin(&[0, 2], &[3, 3]).unwrap(), 2);
        assert_eq!(cart_to_lin(&[1, 2], &[3, 3]).unwrap(), 5);
        assert_eq!(cart_to_lin(&[2, 2], &[3, 3]).unwrap(), 8);
    }
    {
        // 5 x 2 matrix with five columns and two rows with the following linear indexing order:
        // [0 5]
        // [1 6]
        // [2 7]
        // [3 8]
        // [4 9]
        assert_eq!(cart_to_lin(&[0, 0], &[2, 5]).unwrap(), 0);
        assert_eq!(cart_to_lin(&[1, 0], &[2, 5]).unwrap(), 5);
        assert_eq!(cart_to_lin(&[0, 1], &[2, 5]).unwrap(), 1);
        assert_eq!(cart_to_lin(&[1, 1], &[2, 5]).unwrap(), 6);
        assert_eq!(cart_to_lin(&[0, 2], &[2, 5]).unwrap(), 2);
        assert_eq!(cart_to_lin(&[1, 2], &[2, 5]).unwrap(), 7);
        assert_eq!(cart_to_lin(&[0, 3], &[2, 5]).unwrap(), 3);
        assert_eq!(cart_to_lin(&[1, 3], &[2, 5]).unwrap(), 8);
        assert_eq!(cart_to_lin(&[0, 4], &[2, 5]).unwrap(), 4);
        assert_eq!(cart_to_lin(&[1, 4], &[2, 5]).unwrap(), 9);
    }
}

#[test]
fn test_cart_to_lin_3d() {
    {
        // 3 x 2 x 4 matrix with the following linear indexing order:
        // [ 0 12] [ 1 13] [ 2 14] [ 3 15]
        // [ 4 16] [ 5 17] [ 6 18] [ 7 19]
        // [ 8 20] [ 9 21] [10 22] [11 23]
        assert_eq!(cart_to_lin(&[0, 0, 0], &[2, 3, 4]).unwrap(), 0);
        assert_eq!(cart_to_lin(&[0, 0, 1], &[2, 3, 4]).unwrap(), 1);
        assert_eq!(cart_to_lin(&[0, 0, 2], &[2, 3, 4]).unwrap(), 2);
        assert_eq!(cart_to_lin(&[0, 0, 3], &[2, 3, 4]).unwrap(), 3);
        assert_eq!(cart_to_lin(&[1, 2, 3], &[2, 3, 4]).unwrap(), 23);
        assert_eq!(cart_to_lin(&[0, 1, 2], &[2, 3, 4]).unwrap(), 6);
        assert_eq!(cart_to_lin(&[1, 1, 2], &[2, 3, 4]).unwrap(), 18);

        assert!(cart_to_lin(&[2, 1, 2], &[2, 3, 4]).is_none());
        assert!(cart_to_lin(&[54, 1, 2], &[2, 3, 4]).is_none());
        assert!(cart_to_lin(&[1, 3, 2], &[2, 3, 4]).is_none());
        assert!(cart_to_lin(&[1, 0, 5], &[2, 3, 4]).is_none());
    }
}

#[test]
fn test_lin_to_cart_1d() {
    let cart_idx = lin_to_cart(0, &[5]);
    assert_eq!([0], cart_idx.unwrap());

    let cart_idx = lin_to_cart(4, &[5]);
    assert_eq!([4], cart_idx.unwrap());

    let cart_idx = lin_to_cart(5, &[5]);
    assert!(cart_idx.is_none());

    let cart_idx = lin_to_cart(7, &[5]);
    assert!(cart_idx.is_none());
}

#[test]
fn test_lin_to_cart_2d() {
    let cart_idx = lin_to_cart(0, &[2, 3]);
    assert_eq!([0, 0], cart_idx.unwrap());
    let cart_idx = lin_to_cart(1, &[2, 3]);
    assert_eq!([0, 1], cart_idx.unwrap());
    let cart_idx = lin_to_cart(2, &[2, 3]);
    assert_eq!([0, 2], cart_idx.unwrap());
    let cart_idx = lin_to_cart(3, &[2, 3]);
    assert_eq!([1, 0], cart_idx.unwrap());
    let cart_idx = lin_to_cart(4, &[2, 3]);
    assert_eq!([1, 1], cart_idx.unwrap());
    let cart_idx = lin_to_cart(5, &[2, 3]);
    assert_eq!([1, 2], cart_idx.unwrap());
    let cart_idx = lin_to_cart(6, &[2, 3]);
    assert!(cart_idx.is_none());
    let cart_idx = lin_to_cart(1243, &[2, 3]);
    assert!(cart_idx.is_none());
}

#[test]
fn test_lin_to_cart_3d() {
    let cart_idx = lin_to_cart(0, &[2, 4, 7]);
    assert_eq!([0, 0, 0], cart_idx.unwrap());
    let cart_idx = lin_to_cart(5, &[2, 4, 7]);
    assert_eq!([0, 0, 5], cart_idx.unwrap());
    let cart_idx = lin_to_cart(7, &[2, 4, 7]);
    assert_eq!([0, 1, 0], cart_idx.unwrap());
    let cart_idx = lin_to_cart(12, &[2, 4, 7]);
    assert_eq!([0, 1, 5], cart_idx.unwrap());
    let cart_idx = lin_to_cart(19, &[2, 4, 7]);
    assert_eq!([0, 2, 5], cart_idx.unwrap());
    let cart_idx = lin_to_cart(21, &[2, 4, 7]);
    assert_eq!([0, 3, 0], cart_idx.unwrap());
    let cart_idx = lin_to_cart(27, &[2, 4, 7]);
    assert_eq!([0, 3, 6], cart_idx.unwrap());
    let cart_idx = lin_to_cart(28, &[2, 4, 7]);
    assert_eq!([1, 0, 0], cart_idx.unwrap());
    let cart_idx = lin_to_cart(29, &[2, 4, 7]);
    assert_eq!([1, 0, 1], cart_idx.unwrap());
    let cart_idx = lin_to_cart(50, &[2, 4, 7]);
    assert_eq!([1, 3, 1], cart_idx.unwrap());
    let cart_idx = lin_to_cart(55, &[2, 4, 7]);
    assert_eq!([1, 3, 6], cart_idx.unwrap());
    let cart_idx = lin_to_cart(1243, &[2, 4, 7]);
    assert!(cart_idx.is_none());
}

#[test]
fn test_lin_to_cart_dyn_2d() {
    let mut arr = [0, 0];
    let buffer = arr.as_mut_slice();

    assert!(lin_to_cart_dyn(0, &[2, 3], buffer).is_ok());
    assert_eq!(&[0, 0], buffer);

    assert!(lin_to_cart_dyn(1, &[2, 3], buffer).is_ok());
    assert_eq!(&[0, 1], buffer);

    assert!(lin_to_cart_dyn(2, &[2, 3], buffer).is_ok());
    assert_eq!(&[0, 2], buffer);

    assert!(lin_to_cart_dyn(3, &[2, 3], buffer).is_ok());
    assert_eq!(&[1, 0], buffer);

    assert!(lin_to_cart_dyn(4, &[2, 3], buffer).is_ok());
    assert_eq!(&[1, 1], buffer);

    assert!(lin_to_cart_dyn(5, &[2, 3], buffer).is_ok());
    assert_eq!(&[1, 2], buffer);

    assert!(!lin_to_cart_dyn(6, &[2, 3], buffer).is_ok());
    assert_eq!(&[1, 2], buffer); // Buffer is not changed

    assert!(!lin_to_cart_dyn(1243, &[2, 3], buffer).is_ok());
    assert_eq!(&[1, 2], buffer); // Buffer is not changed
}

#[test]
fn test_lin_to_cart_dyn_3d() {
    let mut arr = [0, 0, 0];
    let buffer = arr.as_mut_slice();

    assert!(lin_to_cart_dyn(12, &[2, 4, 7], buffer).is_ok());
    assert_eq!(&[0, 1, 5], buffer);
    assert!(lin_to_cart_dyn(29, &[2, 4, 7], buffer).is_ok());
    assert_eq!(&[1, 0, 1], buffer);
    assert!(lin_to_cart_dyn(50, &[2, 4, 7], buffer).is_ok());
    assert_eq!(&[1, 3, 1], buffer);
    assert!(lin_to_cart_dyn(55, &[2, 4, 7], buffer).is_ok());
    assert_eq!(&[1, 3, 6], buffer);
    assert!(lin_to_cart_dyn(1243, &[2, 4, 7], buffer).is_err());
    assert_eq!(&[1, 3, 6], buffer);
}

#[test]
fn test_cart_to_lin_to_cart_2d() {
    let bounds = [2, 4];

    let input = [1, 1];
    let output = lin_to_cart(cart_to_lin(&input, &bounds).unwrap(), &bounds).unwrap();
    assert_eq!(input, output);

    let input = [0, 3];
    let output = lin_to_cart(cart_to_lin(&input, &bounds).unwrap(), &bounds).unwrap();
    assert_eq!(input, output);

    let input = [1, 0];
    let output = lin_to_cart(cart_to_lin(&input, &bounds).unwrap(), &bounds).unwrap();
    assert_eq!(input, output);

    let input = [1, 3];
    let output = lin_to_cart(cart_to_lin(&input, &bounds).unwrap(), &bounds).unwrap();
    assert_eq!(input, output);
}

#[test]
fn test_cart_to_lin_to_cart_3d() {
    let bounds = [2, 4, 4];

    let input = [1, 1, 2];
    let output = lin_to_cart(cart_to_lin(&input, &bounds).unwrap(), &bounds).unwrap();
    assert_eq!(input, output);

    let input = [0, 3, 3];
    let output = lin_to_cart(cart_to_lin(&input, &bounds).unwrap(), &bounds).unwrap();
    assert_eq!(input, output);

    let input = [1, 0, 0];
    let output = lin_to_cart(cart_to_lin(&input, &bounds).unwrap(), &bounds).unwrap();
    assert_eq!(input, output);

    let input = [1, 3, 2];
    let output = lin_to_cart(cart_to_lin(&input, &bounds).unwrap(), &bounds).unwrap();
    assert_eq!(input, output);
}

#[test]
fn test_lin_to_cart_to_lin_2d() {
    let bounds = [2, 4];

    let input = 4;
    let output = cart_to_lin(&lin_to_cart(input, &bounds).unwrap(), &bounds).unwrap();
    assert_eq!(input, output);

    let input = 7;
    let output = cart_to_lin(&lin_to_cart(input, &bounds).unwrap(), &bounds).unwrap();
    assert_eq!(input, output);

    let input = 2;
    let output = cart_to_lin(&lin_to_cart(input, &bounds).unwrap(), &bounds).unwrap();
    assert_eq!(input, output);
}

#[test]
fn test_lin_to_cart_to_lin_3d() {
    let bounds = [2, 4, 7];

    let input = 21;
    let output = cart_to_lin(&lin_to_cart(input, &bounds).unwrap(), &bounds).unwrap();
    assert_eq!(input, output);

    let input = 17;
    let output = cart_to_lin(&lin_to_cart(input, &bounds).unwrap(), &bounds).unwrap();
    assert_eq!(input, output);

    let input = 50;
    let output = cart_to_lin(&lin_to_cart(input, &bounds).unwrap(), &bounds).unwrap();
    assert_eq!(input, output);
}
