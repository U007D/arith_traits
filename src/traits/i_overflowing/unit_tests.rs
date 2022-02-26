use super::*;
use assert2::assert;
use num_traits::identities::One;

#[test]
fn unconditional_recursion_warning_is_a_false_positive() {
    // Given an `IOverflowing` "adder" function
    fn add_one<T>(n: T) -> <T as IOverflowing>::Output
    where
        T: IOverflowing + One,
    {
        n.overflowing_add(T::one())
    }
    let expected = (42, false);

    // When using the `IOverflowing` "adder" function
    let result = add_one(41);

    // Then it does not recurse (i.e. internally calls `T::overflowing_add()`, not
    // `IOverflowing::overflowing_add()` as recursion would be infinite and a stack overflow would result)
    assert!(result == expected);
}
