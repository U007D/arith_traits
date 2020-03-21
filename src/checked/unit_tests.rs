use super::*;
use num_traits::identities::One;

#[test]
fn unconditional_recursion_warning_is_a_false_positive() {
    // given a `Checked` adder
    fn add_one<T>(n: T) -> <T as Checked>::Output
    where
        T: Checked + One,
    {
        n.checked_add(T::one())
    }

    // when using the `Checked` adder
    let res = add_one(i64::max_value());

    // then it does not recurse (i.e. internally calls `T::checked_add()`, not
    // `Checked::checked_add()`)
    assert_eq!(res, None);
}
