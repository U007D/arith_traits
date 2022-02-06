use super::*;
use assert2::assert;
use num_traits::identities::One;

#[test]
fn unconditional_recursion_warning_is_a_false_positive() {
    // Given an `IWrapping` subtracter
    fn sub_one<T>(n: T) -> <T as IWrapping>::Output
    where
        T: IWrapping + One,
    {
        n.wrapping_sub(T::one())
    }
    let expected = i32::MAX;

    // When using the `Wrap` subtracter
    let result = sub_one(i32::MIN);

    // Then it does not recurse (i.e. internally calls `T::wrapping_add()`, not
    // `Wrap::wrapping_add()`)
    assert!(result == expected);
}
