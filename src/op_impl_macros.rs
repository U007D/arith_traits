#[macro_export]

macro_rules! binary_op_impl {
    ($tr:ty, $typ:ty, $ret:ty; $($op:ident),+ $(,)?) => ($(
        // suppress false-positive `unconditional_recursion` warnings; for proof warnings are
        // false-positives, see `unit_tests::unconditional_recursion_warning_is_a_false_positive()`.
        #[allow(unconditional_recursion)]
        #[inline]
        fn $op(self, rhs: $typ) -> $ret {
            Self::$op(self, rhs)
        }
    )*)
}
#[macro_export]
macro_rules! panicking_binary_op_impl {
    ($tr:ty, $typ:ty, $ret:ty; $($op_outer:ident, $op_inner:ident),+ $(,)?) => ($(
        // suppress false-positive `unconditional_recursion` warnings; see
        // `unit_tests::unconditional_recursion_warning_is_a_false_positive()` for proof warnings are false-positives.
        #[allow(unconditional_recursion)]
        #[inline]
        fn $op_outer(self, rhs: $typ) -> $ret {
            Self::$op_inner(self, rhs).unwrap()
        }
    )*)
}

#[macro_export]
macro_rules! panicking_unary_op_impl {
    ($ret:ty; $($op_outer:ident, $op_inner:ident),+ $(,)?) => ($(
        // suppress false positive `unconditional_recursion` warnings; see `unit_tests` for proof
        // suppress false positive `a method with this name may be added to std` warning; (this macro *is* calling the
        // `std` function!)
        #[allow(unconditional_recursion, unstable_name_collisions)]
        #[inline]
        fn $op_outer(self) -> $ret {
            Self::$op_inner(self).unwrap()
        }
    )*)
}

#[macro_export]
macro_rules! unary_op_impl {
    ($ret:ty; $($op:ident),+ $(,)?) => ($(
        // suppress false positive `unconditional_recursion` warnings; see `unit_tests` for proof
        // suppress false positive `a method with this name may be added to std` warning; (this macro *is* calling the
        // `std` function!)
        #[allow(unconditional_recursion, unstable_name_collisions)]
        #[inline]
        fn $op(self) -> $ret {
            Self::$op(self)
        }
    )*)
}
