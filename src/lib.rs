/*!
A library for converting between linear and cartesian indices.

This library offers the following functions for conversion between linear
and cartesian indices for any number of dimensions:
- [`cart_to_lin`] and [`cart_to_lin_unchecked`]: Convert a cartesian index (e.g. `[1, 2, 5]`
for a three-dimensional matrix) into a linear index (i.e. the position in the
underlying contiguous memory).
- [`lin_to_cart`] and [`lin_to_cart_unchecked`]: Convert a linear index into a cartesian index.
- [`lin_to_cart_dyn`] and [`lin_to_cart_dyn_unchecked`]: These variations of `lin_to_cart`
write the calculated cartesian indices into a caller-provided slice buffer instead of
returning an index array.

Additionally, [`CartesianIndices`] provides an iterator over cartesian indices which can be seen
as the multidimensional equivalent of the [`Range`](<https://doc.rust-lang.org/std/ops/struct.Range.html>) iterator.

# Cartesian to linear index conversion

Let's use the following 2x3 matrix (two rows, three columns) as an example:

```bash
0 1 2
3 4 5
```

The cartesian index of element `0` is `[0, 0]`, that of `1` is `[0, 1]`, that of `5` is `[1, 2]` and so on.
`cart_to_lin` (as well as all other functions of this library) uses row-major order
(= last index changes fastest).
```
use cart_lin::cart_to_lin;

// Rows, columns
let dim_size = [2, 3];

assert_eq!(cart_to_lin(&[0, 0], &dim_size).unwrap(), 0);
assert_eq!(cart_to_lin(&[0, 1], &dim_size).unwrap(), 1);
assert_eq!(cart_to_lin(&[0, 2], &dim_size).unwrap(), 2);
assert_eq!(cart_to_lin(&[1, 0], &dim_size).unwrap(), 3);
assert_eq!(cart_to_lin(&[1, 1], &dim_size).unwrap(), 4);
assert_eq!(cart_to_lin(&[1, 2], &dim_size).unwrap(), 5);

```

For higher-dimensional matrices, it works in the same way (using the example of a matrix
with 4 rows, 3 columns and 2 pages):
```
use cart_lin::cart_to_lin;

// Rows, columns, pages
let dim_size = [4, 3, 2];

assert_eq!(cart_to_lin(&[0, 0, 0], &dim_size).unwrap(), 0);
assert_eq!(cart_to_lin(&[0, 0, 1], &dim_size).unwrap(), 1);
assert_eq!(cart_to_lin(&[0, 1, 0], &dim_size).unwrap(), 2);
assert_eq!(cart_to_lin(&[0, 1, 1], &dim_size).unwrap(), 3);
assert_eq!(cart_to_lin(&[0, 2, 0], &dim_size).unwrap(), 4);
assert_eq!(cart_to_lin(&[0, 2, 1], &dim_size).unwrap(), 5);
assert_eq!(cart_to_lin(&[1, 0, 0], &dim_size).unwrap(), 6);
```
[`cart_to_lin`] checks whether the given cartesian index is valid for the specified number of dimensions.
In order to avoid this check, use [`cart_to_lin_unchecked`] (which is not unsafe, but might return
invalid indices).

# Linear to cartesian conversion

The inverse of [`cart_to_lin`] is [`lin_to_cart`]:
```
use cart_lin::lin_to_cart;

// Rows, columns
let dim_size = [2, 3];

assert_eq!(lin_to_cart(0, &dim_size).unwrap(), [0, 0]);
assert_eq!(lin_to_cart(1, &dim_size).unwrap(), [0, 1]);
assert_eq!(lin_to_cart(2, &dim_size).unwrap(), [0, 2]);
assert_eq!(lin_to_cart(3, &dim_size).unwrap(), [1, 0]);
assert_eq!(lin_to_cart(4, &dim_size).unwrap(), [1, 1]);
assert_eq!(lin_to_cart(5, &dim_size).unwrap(), [1, 2]);
```

# Iterate over cartesian indices

```
use cart_lin::CartesianIndices;

let mut cartiter = CartesianIndices::new([2, 3]);
assert_eq!(cartiter.next(), Some([0, 0]));
assert_eq!(cartiter.next(), Some([0, 1]));
assert_eq!(cartiter.next(), Some([0, 2]));
assert_eq!(cartiter.next(), Some([1, 0]));
assert_eq!(cartiter.next(), Some([1, 1]));
assert_eq!(cartiter.next(), Some([1, 2]));
assert_eq!(cartiter.next(), None);
```

[`CartesianIndices`] can also be constructed by defining lower and upper bounds for each axis.
The following example is functionally equivalent to the previous one:
```
use cart_lin::CartesianIndices;

let mut cartiter = CartesianIndices::from_bounds([[0, 2], [0, 3]]).expect("bounds must be strictly monotonic increasing");
assert_eq!(cartiter.next(), Some([0, 0]));
assert_eq!(cartiter.next(), Some([0, 1]));
assert_eq!(cartiter.next(), Some([0, 2]));
assert_eq!(cartiter.next(), Some([1, 0]));
assert_eq!(cartiter.next(), Some([1, 1]));
assert_eq!(cartiter.next(), Some([1, 2]));
assert_eq!(cartiter.next(), None);
```

But it is also possible to add offsets via the lower bounds:
```
use cart_lin::CartesianIndices;

let mut cartiter = CartesianIndices::from_bounds([[1, 3], [2, 5]]).expect("bounds must be strictly monotonic increasing");
assert_eq!(cartiter.next(), Some([1, 2]));
assert_eq!(cartiter.next(), Some([1, 3]));
assert_eq!(cartiter.next(), Some([1, 4]));
assert_eq!(cartiter.next(), Some([2, 2]));
assert_eq!(cartiter.next(), Some([2, 3]));
assert_eq!(cartiter.next(), Some([2, 4]));
assert_eq!(cartiter.next(), None);
```
*/

/**
Check whether the given indices are valid. This is the case if the length of `indices`
is equal to the dimensionality of the data `N`, and if all individual axes indices are in bounds.
*/
fn valid_indices(indices: &[usize], dim_size: &[usize]) -> bool {
    for (cart_index, bound) in indices.iter().zip(dim_size.iter()) {
        if *cart_index >= *bound {
            return false;
        }
    }
    return indices.len() == dim_size.len();
}

/**
Convert a cartesian index into a linear index (row-major).

This function takes two arguments -- cartesian indices and the size of each dimension as slices -- and uses them
to calculate the corresponding linear index in row-major order. If the length of the cartesian index is not
equal to the number of dimensions (= length of `dim_size`), or if any of the cartesian indices are out of bounds,
this function returns `None`.
```
use cart_lin::cart_to_lin;

// 2 x 5 matrix with five columns and two rows
let dim_size = [2, 5];
assert_eq!(cart_to_lin(&[0, 0], &dim_size).unwrap(), 0);
assert_eq!(cart_to_lin(&[0, 1], &dim_size).unwrap(), 1);
assert_eq!(cart_to_lin(&[0, 2], &dim_size).unwrap(), 2);
// ...
assert_eq!(cart_to_lin(&[1, 3], &dim_size).unwrap(), 8);
assert_eq!(cart_to_lin(&[1, 4], &dim_size).unwrap(), 9);

// Out-of-bounds cartesian indices:
assert!(cart_to_lin(&[1, 5], &dim_size).is_none()); // matrix has five columns, hence the maximum column index is 4.

// 2 x 3 x 4 matrix
let dim_size = [2, 3, 4];
assert_eq!(cart_to_lin(&[0, 0, 0], &dim_size).unwrap(), 0);
assert_eq!(cart_to_lin(&[0, 0, 1], &dim_size).unwrap(), 1);
assert_eq!(cart_to_lin(&[0, 0, 2], &dim_size).unwrap(), 2);
// ...
assert_eq!(cart_to_lin(&[0, 1, 0], &dim_size).unwrap(), 4);
assert_eq!(cart_to_lin(&[0, 1, 1], &dim_size).unwrap(), 5);
assert_eq!(cart_to_lin(&[0, 1, 2], &dim_size).unwrap(), 6);
// ...
assert_eq!(cart_to_lin(&[1, 0, 0], &dim_size).unwrap(), 12);
assert_eq!(cart_to_lin(&[1, 0, 1], &dim_size).unwrap(), 13);
assert_eq!(cart_to_lin(&[1, 0, 2], &dim_size).unwrap(), 14);
```
*/
pub fn cart_to_lin(indices: &[usize], dim_size: &[usize]) -> Option<usize> {
    if valid_indices(indices, dim_size) {
        return Some(cart_to_lin_unchecked(indices, dim_size));
    } else {
        return None;
    }
}

/**
Like [`cart_to_lin`], but without the checks.

Despite the name, this function itself is safe. However, the index received from this function might be invalid. Using
such an invalid index to perform an unsafe operation on a matrix structure of a matrix library (e.g. `matrix.get_unchecked`)
causes an out-of-bounds read and is therefore undefined behaviour.
```
use cart_lin::{cart_to_lin, cart_to_lin_unchecked};

// 2 x 5 matrix with five columns and two rows
let dim_size = [2, 5];

// Valid cartesian indices
assert_eq!(cart_to_lin(&[1, 4], &dim_size).unwrap(), 9);
assert_eq!(cart_to_lin_unchecked(&[1, 4], &dim_size), 9);

// Out-of-bounds cartesian indices:
assert!(cart_to_lin(&[1, 5], &dim_size).is_none());
assert_eq!(cart_to_lin_unchecked(&[1, 5], &dim_size), 10); // Nonsensical value - matrix only has 10 entries (linear index 0 to 9)!
```
*/
pub fn cart_to_lin_unchecked(indices: &[usize], dim_size: &[usize]) -> usize {
    let mut index: usize = 0;
    let mut multiplier: usize = 1;
    for (cart_index, bound) in indices.iter().rev().zip(dim_size.iter().rev()) {
        index += multiplier * cart_index;
        multiplier *= *bound;
    }
    return index;
}

/**
Convert a linear index to a cartesian index (row-major).

This function takes the linear index and the size of each dimension as a slice and uses them to
calculate the corresponding cartesian index. If the linear index is out of bounds (= equal to
or larger than the cartiter of all values in the `dim_size` vector), this function returns `None`.
```
use cart_lin::lin_to_cart;

let dim_size = [2, 3];
assert_eq!([0, 2], lin_to_cart(2, &dim_size).unwrap());
assert_eq!([1, 1], lin_to_cart(4, &dim_size).unwrap());
assert!(lin_to_cart(6, &dim_size).is_none()); // Out of bounds
```
 */
pub fn lin_to_cart<const N: usize>(index: usize, dim_size: &[usize; N]) -> Option<[usize; N]> {
    if index >= dim_size.iter().fold(1, |acc, bound| acc * bound) {
        return None;
    } else {
        return Some(lin_to_cart_unchecked(index, dim_size));
    }
}

/**
Like [`lin_to_cart`], but without the checks.

Despite the name, this function itself is safe. However, the index received from this function might be invalid. Using
such an invalid index to perform an unsafe operation on a matrix structure of a matrix library (e.g. `matrix.get_unchecked`)
causes an out-of-bounds read and is therefore undefined behaviour.
```
use cart_lin::{lin_to_cart, lin_to_cart_unchecked};

let dim_size = [2, 3];

// Valid linear indices
assert_eq!([0, 2], lin_to_cart(2, &dim_size).unwrap());
assert_eq!([0, 2], lin_to_cart_unchecked(2, &dim_size));

// Out-of-bounds linear indices:
assert!(lin_to_cart(6, &dim_size).is_none()); // Out of bounds
assert_eq!([0, 0], lin_to_cart_unchecked(6, &dim_size)); // Nonsensical value (wrapping around)
```
 */
pub fn lin_to_cart_unchecked<const N: usize>(index: usize, dim_size: &[usize; N]) -> [usize; N] {
    let mut indices = [0; N];
    lin_to_cart_dyn_unchecked(index, dim_size, indices.as_mut_slice());
    return indices;
}

/**
Like [`lin_to_cart`], but mutates `cart_indices` in place instead of returning a new array.

While [`lin_to_cart`] expects arrays (size known at compile time), this function works with slices
(whose length is not known until runtime and may dynamically change).
If the length of `bounds` and `cart_indices` is not identical or the linear index is out of bounds, this function
returns an error (and does not change `cart_indices`).
```
use cart_lin::lin_to_cart_dyn;

let dim_size = vec![2, 3];
let mut indices = vec![0, 0];

assert!(lin_to_cart_dyn(2, dim_size.as_slice(), indices.as_mut_slice()).is_ok());
assert_eq!(&[0, 2], indices.as_slice());

assert!(lin_to_cart_dyn(4, dim_size.as_slice(), indices.as_mut_slice()).is_ok());
assert_eq!(&[1, 1], indices.as_slice());

// Incorrect usage: indices vector is too short -> Error
let mut indices = vec![0];
assert!(lin_to_cart_dyn(4, dim_size.as_slice(), indices.as_mut_slice()).is_err());
```
 */
pub fn lin_to_cart_dyn(
    index: usize,
    dim_size: &[usize],
    cart_indices: &mut [usize],
) -> Result<(), &'static str> {
    if dim_size.len() != cart_indices.len()
        || index >= dim_size.iter().fold(1, |acc, bound| acc * bound)
    {
        return Err("length of slices not equal or index out of bounds");
    } else {
        lin_to_cart_dyn_unchecked(index, dim_size, cart_indices);
        return Ok(());
    }
}

/**
Like [`lin_to_cart_dyn`], but without the checks.
```
use cart_lin::lin_to_cart_dyn_unchecked;

let dim_size = vec![2, 3];
let mut indices = vec![0, 0];

// Correct usage
lin_to_cart_dyn_unchecked(4, dim_size.as_slice(), indices.as_mut_slice());
assert_eq!(&[1, 1], indices.as_slice());

// Incorrect usage: indices vector is too short, hence only the first value is populated
let mut indices = vec![0];
lin_to_cart_dyn_unchecked(4, dim_size.as_slice(), indices.as_mut_slice());
assert_eq!(&[1], indices.as_slice());
```
 */
pub fn lin_to_cart_dyn_unchecked(index: usize, dim_size: &[usize], cart_indices: &mut [usize]) {
    // Make the index mutable
    let mut index = index;

    // Fill up the indices from back to front by performing modulo and truncating integer divisons
    for (idx, bound) in cart_indices.iter_mut().rev().zip(dim_size.iter().rev()) {
        let remainder = index % *bound;
        index = index / *bound;
        *idx = remainder;
    }
}

/**
An iterator over all cartesian indices within the input dimension sizes.
 */
#[derive(Debug)]
pub struct CartesianIndices<const N: usize> {
    current: usize,
    max: usize,
    limit_deltas: [usize; N],
    bounds: [[usize; 2]; N],
}

impl<const N: usize> CartesianIndices<N> {
    /**
    Creates a new `CartesianIndices` iterator using the given dimension sizes.
    ```
    use cart_lin::CartesianIndices;

    let mut cartiter = CartesianIndices::new([3]);
    assert_eq!(cartiter.next(), Some([0]));
    assert_eq!(cartiter.next(), Some([1]));
    assert_eq!(cartiter.next(), Some([2]));
    assert_eq!(cartiter.next(), None);

    let mut cartiter = CartesianIndices::new([1, 3]);
    assert_eq!(cartiter.next(), Some([0, 0]));
    assert_eq!(cartiter.next(), Some([0, 1]));
    assert_eq!(cartiter.next(), Some([0, 2]));
    assert_eq!(cartiter.next(), None);
    ```
     */
    pub fn new(dim_size: [usize; N]) -> Self {
        let mut bounds = [[0, 0]; N];
        for (limits, dim) in bounds.iter_mut().zip(dim_size.into_iter()) {
            limits[1] = dim;
        }

        return Self::with_offsets_unchecked(bounds);
    }

    /**
    Creates a new [`CartesianIndices`] using lower and upper bounds of each dimension.

    The lower and upper bounds must be given as an two-element array and the lower
    bound must be smaller than or equal to the upper bound:
    ```
    use cart_lin::CartesianIndices;

    // Valid input:
    // Indices for first dimension are between 1 and 3 (excluded)
    // Indices for second dimension are between 2 and 3 (excluded)
    let mut cartiter = CartesianIndices::from_bounds([[1, 3], [2, 5]]).expect("bounds must be strictly monotonic increasing");
    assert_eq!(cartiter.next(), Some([1, 2]));
    assert_eq!(cartiter.next(), Some([1, 3]));
    assert_eq!(cartiter.next(), Some([1, 4]));
    assert_eq!(cartiter.next(), Some([2, 2]));
    assert_eq!(cartiter.next(), Some([2, 3]));
    assert_eq!(cartiter.next(), Some([2, 4]));
    assert_eq!(cartiter.next(), None);

    // Invalid input:
    // Lower bound for first dimension is 1, but upper bound is 0?
    assert!(CartesianIndices::from_bounds([[1, 0], [2, 3]]).is_none());

    // Invalid input:
    // Lower bound for first dimension is 1, but upper bound is also 1?
    assert!(CartesianIndices::from_bounds([[1, 1], [2, 3]]).is_none());
    ```
     */
    pub fn from_bounds(bounds: [[usize; 2]; N]) -> Option<Self> {
        for index_limits in bounds.iter() {
            if index_limits[1] <= index_limits[0] {
                return None;
            }
        }

        return Some(Self::with_offsets_unchecked(bounds));
    }

    /**
    Like [`Self::from_bounds`], but without the checks.

    Despite the name, this function itself is safe. However, the index received from this function might be invalid. Using
    such an invalid index to perform an unsafe operation on a matrix structure of a matrix library (e.g. `matrix.get_unchecked`)
    causes an out-of-bounds read and is therefore undefined behaviour.
    */
    pub fn with_offsets_unchecked(bounds: [[usize; 2]; N]) -> Self {
        let mut max = 1;
        let mut limit_deltas = [0; N];
        for (limits, delta) in bounds.iter().zip(limit_deltas.iter_mut()) {
            *delta = limits[1] - limits[0];
            max = max * *delta;
        }

        return Self {
            current: 0,
            max,
            limit_deltas,
            bounds,
        };
    }
}

impl<const N: usize> Iterator for CartesianIndices<N> {
    type Item = [usize; N];

    fn next(&mut self) -> Option<Self::Item> {
        if self.current == self.max {
            return None;
        }

        // Calculate the linear indices
        let mut res = lin_to_cart_unchecked(self.current, &self.limit_deltas);

        // Add offsets from lower limits
        for (r, limits) in res.iter_mut().zip(self.bounds.iter()) {
            *r += limits[0];
        }

        self.current += 1;
        return Some(res);
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.current = n;
        return self.next();
    }
}
