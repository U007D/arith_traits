// suppress `use_self` recommendation; unavoidable in macro context
#![allow(clippy::use_self)]
#[cfg(test)]
mod unit_tests;

pub trait Wrapping<T = Self> {
    type Output;

    fn wrapping_abs(self) -> Self::Output;
    fn wrapping_add(self, rhs: T) -> Self::Output;
    fn wrapping_div(self, rhs: T) -> Self::Output;
    fn wrapping_div_euclid(self, rhs: T) -> Self::Output;
    fn wrapping_mul(self, rhs: T) -> Self::Output;
    fn wrapping_neg(self) -> Self::Output;
    fn wrapping_pow(self, rhs: u32) -> Self::Output;
    fn wrapping_rem(self, rhs: T) -> Self::Output;
    fn wrapping_rem_euclid(self, rhs: T) -> Self::Output;
    fn wrapping_shl(self, rhs: u32) -> Self::Output;
    fn wrapping_shr(self, rhs: u32) -> Self::Output;
    fn wrapping_sub(self, rhs: T) -> Self::Output;
}

macro_rules! binary_op_impl {
    ($t:ty, $($f:ident),*) => ($(
        // suppress false positive `unconditional_recursion` warnings; see `unit_tests` for proof
        #[allow(unconditional_recursion)]
        #[inline]
        fn $f(self, rhs: $t) -> Self::Output {
            Self::$f(self, rhs)
        }
    )*)
}

macro_rules! binary_op_rhs_u32_impl {
    ($t:ty, $($f:ident),*) => ($(
        // suppress false positive `unconditional_recursion` warnings; see `unit_tests` for proof
        #[allow(unconditional_recursion)]
        #[inline]
        fn $f(self, rhs: $t) -> Self::Output {
            Self::$f(self, rhs)
        }
    )*)
}

macro_rules! unary_op_impl {
    ($($f:ident),*) => ($(
        // suppress false positive `unconditional_recursion` warnings; see `unit_tests` for proof
        #[allow(unconditional_recursion)]
        #[inline]
        fn $f(self) -> Self::Output {
            Self::$f(self)
        }
    )*)
}

macro_rules! wrapping_impl {
    ($($t:ty)*) => ($(
        impl Wrapping for $t {
            type Output = Self;

            binary_op_impl! {
                $t,
                wrapping_add,
                wrapping_div,
                wrapping_div_euclid,
                wrapping_mul,
                wrapping_rem,
                wrapping_rem_euclid,
                wrapping_sub
            }

            binary_op_rhs_u32_impl! {
                u32,
                wrapping_pow,
                wrapping_shl,
                wrapping_shr
            }

            unary_op_impl! {
                wrapping_abs,
                wrapping_neg
            }
        }
    )*)
}

wrapping_impl! { i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize }
