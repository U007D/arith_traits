use super::*;
use num_traits::identities::One;

#[test]
fn unconditional_recursion_warning_is_a_false_positive() {
    // given a `Wrap` subtracter
    fn sub_one<T>(n: T) -> <T as Wrap>::Output
    where
        T: Wrap + One,
    {
        n.wrapping_sub(T::one())
    }

    // when using the `Wrap` subtracter
    let res = sub_one(i32::min_value());

    // then it does not recurse (i.e. internally calls `T::wrapping_add()`, not
    // `Wrap::wrapping_add()`)
    assert_eq!(res, i32::max_value());
}
