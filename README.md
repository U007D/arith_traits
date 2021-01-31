# arith_traits

Traits unifying types based on various supported arithmetic operations.

If you've ever wanted your generic function to accept any value that supports, for example,
checked arithmetic, you have discovered that `std` does not define a `trait` that all checked types
implement.

This crate remedies that.  It was developed to simplify the implementation of the
[Ranged](https://crates.io/crates/ranged) crate.  It may also be of value to other crates performing
generic arithmetic.

This crate is currently at the PoC stage and is very incomplete.

## License
Licensed under either:
* MIT license (see LICENSE-MIT file)
* Apache License, Version 2.0 (see LICENSE-APACHE file)
at your option.

## Contributions
Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the
work by you shall be dual licensed as above, without any additional terms or conditions.

