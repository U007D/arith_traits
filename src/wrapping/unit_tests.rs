use super::*;
use num_traits::identities::One;

#[test]
fn unconditional_recursion_warning_is_a_false_positive() {
    // given a `Wrapping` subtracter
    fn sub_one<T>(n: T) -> <T as Wrapping>::Output
    where
        T: Wrapping + One,
    {
        n.wrapping_sub(T::one())
    }

    // when using the `Wrapping` subtracter
    let res = sub_one(i32::min_value());

    // then it does not recurse (i.e. internally calls `T::wrapping_add()`, not
    // `Wrapping::wrapping_add()`)
    assert_eq!(res, i32::max_value());
}
