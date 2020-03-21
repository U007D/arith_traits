use super::*;
use num_traits::identities::One;

#[test]
fn unconditional_recursion_warning_is_a_false_positive() {
    // given an `Overflow` adder
    fn add_one<T>(n: T) -> <T as Overflow>::Output
    where
        T: Overflow + One,
    {
        n.overflowing_add(T::one())
    }

    // when using the `Overflow` adder
    let res = add_one(41);

    // then it does not recurse (i.e. internally calls `T::overflowing_add()`, not
    // `Overflow::overflowing_add()`)
    assert_eq!(res, (42, false));
}
