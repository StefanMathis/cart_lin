# cart_lin

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

## Cartesian to linear index conversion

Let's use the following 2x3 matrix (two rows, three columns) as an example:

```bash
0 1 2
3 4 5
```

The cartesian index of element `0` is `[0, 0]`, that of `1` is `[0, 1]`, that of `5` is `[1, 2]` and so on.
`cart_to_lin` (as well as all other functions of this library) uses row-major order
(= last index changes fastest).
```rust
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
```rust
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

## Linear to cartesian conversion

The inverse of [`cart_to_lin`] is [`lin_to_cart`]:
```rust
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

## Iterate over cartesian indices

```rust
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
```rust
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
```rust
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