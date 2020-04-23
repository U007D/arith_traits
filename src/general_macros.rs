#[macro_export]
macro_rules! binary_op_impl {
    ($t:ty, $($f:ident),*) => ($(
        // suppress false-positive `unconditional_recursion` warnings; see
        // `unit_tests::unconditional_recursion_warning_is_a_false_positive()` for proof warnings
        // are false-positives.
        #[allow(unconditional_recursion)]
        #[inline]
        fn $f(self, rhs: $t) -> Self::Output {
            Self::$f(self, rhs)
        }
    )*)
}

#[macro_export]
macro_rules! unary_op_impl {
    ($($f:ident),*) => ($(
        // suppress false positive `unconditional_recursion` warnings; see `unit_tests` for proof
        // suppress false positive `a method with this name may be added to std` warning;
        //     (this macro is calling the std function now!)
        #[allow(unconditional_recursion, unstable_name_collisions)]
        #[inline]
        fn $f(self) -> Self::Output {
            Self::$f(self)
        }
    )*)
}
