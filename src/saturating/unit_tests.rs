use super::*;
use num_traits::identities::One;

#[test]
fn unconditional_recursion_warning_is_a_false_positive() {
    // given a `Saturating` subtracter
    fn sub_one<T>(n: T) -> <T as Saturating>::Output
    where
        T: Saturating + One,
    {
        n.saturating_sub(T::one())
    }

    // when using the `Saturating` subtracter
    let res = sub_one(i128::min_value());

    // then it does not recurse (i.e. internally calls `T::saturating_add()`, not
    // `Saturating::saturating_add()`)
    assert_eq!(res, i128::min_value());
}
