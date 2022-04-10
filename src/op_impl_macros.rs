#[macro_export]

// TODO: Use UFCS instead of suppressing linter false-positives to disambiguate between trait methods and inherent
//       methods of the same name.  Improves readability to the user without context.
macro_rules! binary_op_impl {
    ($typ:ty, $($op:ident),*) => ($(
        // suppress false-positive `unconditional_recursion` warnings; see
        // `unit_tests::unconditional_recursion_warning_is_a_false_positive()` for proof warnings are false-positives.
        #[allow(unconditional_recursion)]
        #[inline]
        fn $op(self, rhs: $typ) -> Self::Output {
            Self::$op(self, rhs)
        }
    )*)
}

#[macro_export]
macro_rules! panicking_binary_op_impl {
    ($typ:ty, $($op_outer:ident, $op_inner:ident),*) => ($(
        // suppress false-positive `unconditional_recursion` warnings; see
        // `unit_tests::unconditional_recursion_warning_is_a_false_positive()` for proof warnings are false-positives.
        #[allow(unconditional_recursion)]
        #[inline]
        fn $op_outer(self, rhs: $typ) -> Self::Output {
            Self::$op_inner(self, rhs).unwrap()
        }
    )*)
}

#[macro_export]
macro_rules! panicking_unary_op_impl {
    ($($op_outer:ident, $op_inner:ident),*) => ($(
        // suppress false positive `unconditional_recursion` warnings; see `unit_tests` for proof
        // suppress false positive `a method with this name may be added to std` warning; (this macro *is* calling the
        // `std` function!)
        #[allow(unconditional_recursion, unstable_name_collisions)]
        #[inline]
        fn $op_outer(self) -> <Self as IPanickingOps>::Output {
            Self::$op_inner(self).unwrap()
        }
    )*)
}

#[macro_export]
macro_rules! unary_op_impl {
    ($($op:ident),*) => ($(
        // suppress false positive `unconditional_recursion` warnings; see `unit_tests` for proof
        // suppress false positive `a method with this name may be added to std` warning; (this macro *is* calling the
        // `std` function!)
        #[allow(unconditional_recursion, unstable_name_collisions)]
        #[inline]
        fn $op(self) -> Self::Output {
            Self::$op(self)
        }
    )*)
}
