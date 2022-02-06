use super::*;
use assert2::assert;
use num_traits::identities::One;

#[test]
fn unconditional_recursion_warning_is_a_false_positive() {
    // Given an `ISaturating` subtracter
    fn sub_one<T>(n: T) -> <T as ISaturating>::Output
    where
        T: ISaturating + One,
    {
        n.saturating_sub(T::one())
    }
    let expected = i128::MIN;

    // When using the `Saturate` subtracter
    let result = sub_one(i128::MIN);

    // Then it does not recurse (i.e. internally calls `T::saturating_add()`, not
    // `Saturate::saturating_add()`)
    assert!(result == expected);
}
