use super::*;
use assert2::assert;
use num_traits::identities::One;

#[test]
fn unconditional_recursion_warning_is_a_false_positive() {
    // Given an `IWrapping` "subtracter" function
    fn sub_one<T>(n: T) -> <T as IWrappingOps>::Output
    where
        T: IWrappingOps + One,
    {
        n.wrapping_sub(T::one())
    }
    let expected = i32::MAX;

    // When using the `IWrapping` "subtracter" function
    let result = sub_one(i32::MIN);

    // Then it does not recurse (i.e. internally calls `T::wrapping_add()`, not
    // `IWrapping::wrapping_sub()` as recursion would be infinite and a stack overflow would result)
    assert!(result == expected);
}
