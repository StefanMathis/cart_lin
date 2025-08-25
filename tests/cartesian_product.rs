use cart_lin::CartesianIndices;

#[test]
fn test_cartesian_product_2d() {
    {
        let mut product = CartesianIndices::new([2, 3]);
        assert_eq!(product.next(), Some([0, 0]));
        assert_eq!(product.next(), Some([0, 1]));
        assert_eq!(product.next(), Some([0, 2]));
        assert_eq!(product.next(), Some([1, 0]));
        assert_eq!(product.next(), Some([1, 1]));
        assert_eq!(product.next(), Some([1, 2]));
        assert_eq!(product.next(), None);
    }
    {
        let mut product = CartesianIndices::from_bounds([[0, 2], [0, 3]]).unwrap();
        assert_eq!(product.next(), Some([0, 0]));
        assert_eq!(product.next(), Some([0, 1]));
        assert_eq!(product.next(), Some([0, 2]));
        assert_eq!(product.next(), Some([1, 0]));
        assert_eq!(product.next(), Some([1, 1]));
        assert_eq!(product.next(), Some([1, 2]));
        assert_eq!(product.next(), None);
    }
    assert!(CartesianIndices::from_bounds([[0, 0], [0, 3]]).is_none());
    assert!(CartesianIndices::from_bounds([[1, 0], [0, 3]]).is_none());
    assert!(CartesianIndices::from_bounds([[1, 0], [3, 3]]).is_none());
    assert!(CartesianIndices::from_bounds([[1, 0], [3, 2]]).is_none());
}

#[test]
fn test_cartesian_product_3d() {
    {
        // Cartesian product with three elements
        let mut product = CartesianIndices::new([1, 2, 3]);
        assert_eq!(product.next(), Some([0, 0, 0]));
        assert_eq!(product.next(), Some([0, 0, 1]));
        assert_eq!(product.next(), Some([0, 0, 2]));
        assert_eq!(product.next(), Some([0, 1, 0]));
        assert_eq!(product.next(), Some([0, 1, 1]));
        assert_eq!(product.next(), Some([0, 1, 2]));
        assert_eq!(product.next(), None);
    }
    {
        // Cartesian product with three elements
        let mut product = CartesianIndices::new([2, 2, 3]);
        assert_eq!(product.next(), Some([0, 0, 0]));
        assert_eq!(product.next(), Some([0, 0, 1]));
        assert_eq!(product.next(), Some([0, 0, 2]));
        assert_eq!(product.next(), Some([0, 1, 0]));
        assert_eq!(product.next(), Some([0, 1, 1]));
        assert_eq!(product.next(), Some([0, 1, 2]));
        assert_eq!(product.next(), Some([1, 0, 0]));
        assert_eq!(product.next(), Some([1, 0, 1]));
        assert_eq!(product.next(), Some([1, 0, 2]));
        assert_eq!(product.next(), Some([1, 1, 0]));
        assert_eq!(product.next(), Some([1, 1, 1]));
        assert_eq!(product.next(), Some([1, 1, 2]));
        assert_eq!(product.next(), None);
    }
}
