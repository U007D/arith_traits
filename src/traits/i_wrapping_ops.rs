// suppress `use_self` recommendation; unavoidable in macro context
#![allow(clippy::use_self)]

#[cfg(test)]
mod unit_tests;

pub trait IWrappingOps<T = Self> {
    type Output/*: Into<WrappingPolicy<T>>*/;

    #[must_use]
    fn wrapping_abs(self) -> Self::Output;
    #[must_use]
    fn wrapping_add(self, rhs: T) -> Self::Output;
    #[must_use]
    fn wrapping_div(self, rhs: T) -> Self::Output;
    #[must_use]
    fn wrapping_div_euclid(self, rhs: T) -> Self::Output;
    #[must_use]
    fn wrapping_mul(self, rhs: T) -> Self::Output;
    #[must_use]
    fn wrapping_neg(self) -> Self::Output;
    #[must_use]
    fn wrapping_pow(self, rhs: u32) -> Self::Output;
    #[must_use]
    fn wrapping_rem(self, rhs: T) -> Self::Output;
    #[must_use]
    fn wrapping_rem_euclid(self, rhs: T) -> Self::Output;
    #[must_use]
    fn wrapping_shl(self, rhs: u32) -> Self::Output;
    #[must_use]
    fn wrapping_shr(self, rhs: u32) -> Self::Output;
    #[must_use]
    fn wrapping_sub(self, rhs: T) -> Self::Output;
}

macro_rules! wrapping_impl {
    ($tr:ty, $ret:ty; $($t:ty),+ $(,)?) => ($(
        impl IWrappingOps for $t {
            type Output = $ret;

            binary_op_impl! {
                $tr, $t, $ret;
                wrapping_add,
                wrapping_div,
                wrapping_div_euclid,
                wrapping_mul,
                wrapping_rem,
                wrapping_rem_euclid,
                wrapping_sub
            }

            binary_op_impl! {
                $tr, u32, $ret;
                wrapping_pow,
                wrapping_shl,
                wrapping_shr
            }

            unary_op_impl! {
                $ret;
                wrapping_abs,
                wrapping_neg
            }
        }
    )*)
}

wrapping_impl! { IWrappingOps, Self; i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, }
