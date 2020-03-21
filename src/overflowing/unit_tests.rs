use super::*;
use num_traits::identities::One;

#[test]
fn unconditional_recursion_warning_is_a_false_positive() {
    // given an `Overflowing` adder
    fn add_one<T>(n: T) -> <T as Overflowing>::Output
    where
        T: Overflowing + One,
    {
        n.overflowing_add(T::one())
    }

    // when using the `Overflowing` adder
    let res = add_one(41);

    // then it does not recurse (i.e. internally calls `T::overflowing_add()`, not
    // `Overflowing::overflowing_add()`)
    assert_eq!(res, (42, false));
}
