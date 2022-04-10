use super::*;
use assert2::assert;
use num_traits::identities::One;

#[test]
fn unconditional_recursion_warning_is_a_false_positive() {
    // Given an `ISaturating` "subtracter" function
    fn sub_one<T>(n: T) -> <T as ISaturatingOps>::Output
    where
        T: ISaturatingOps + One,
    {
        n.saturating_sub(T::one())
    }
    let expected = i128::MIN;

    // When using the `ISaturating` "subtracter" function
    let result = sub_one(i128::MIN);

    // Then it does not recurse (i.e. internally calls `T::saturating_add()`, not
    // ``ISaturating::saturating_sub()` as recursion would be infinite and a stack overflow would result)
    assert!(result == expected);
}
