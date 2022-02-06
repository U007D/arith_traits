use super::*;
use assert2::assert;
use num_traits::identities::One;
use std::panic;

#[test]
fn unconditional_recursion_warning_is_a_false_positive() {
    // Given an `IPanicking` "adder" function
    fn add_one<T>(n: T) -> <T as IPanicking>::Output
    where
        T: IPanicking + One,
    {
        n.panicking_add(T::one())
    }

    // When using the `Check` "adder" function
    let res = panic::catch_unwind(|| add_one(i64::MAX));

    // Then it does not recurse (i.e. internally calls `T::checked_add()`, not
    // `Check::checked_add()`)
    assert!(res.is_err());
}
