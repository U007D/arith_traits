use super::*;
use num_traits::identities::One;

#[test]
fn unconditional_recursion_warning_is_a_false_positive() {
    // given a `Check` adder
    fn add_one<T>(n: T) -> <T as Check>::Output
    where
        T: Check + One,
    {
        n.checked_add(T::one())
    }

    // when using the `Check` adder
    let res = add_one(i64::max_value());

    // then it does not recurse (i.e. internally calls `T::checked_add()`, not
    // `Check::checked_add()`)
    assert_eq!(res, None);
}
