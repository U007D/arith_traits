/*
                checked_add,
                checked_div,
                checked_div_euclid,
                checked_mul,
                checked_rem,
                checked_rem_euclid,
                checked_sub
 */
use super::*;
use assert2::assert;
use num_traits::identities::One;

#[test]
fn unconditional_recursion_warning_is_a_false_positive() {
    // Given an `IChecked` "adder" function
    fn add_one<T>(n: T) -> <T as IChecked>::Output
    where
        T: IChecked + One,
    {
        n.checked_add(T::one())
    }
    let expected = None;

    // When using the `IChecked` "adder" function
    let result = add_one(i64::MAX);

    // Then it does not recurse (i.e. internally calls `T::checked_add()`, not
    // `IChecked::checked_add()` as recursion would be infinite and a stack overflow would result)
    assert!(result == expected);
}
