// suppress `use_self` recommendation; unavoidable in macro context
#![allow(clippy::use_self)]

#[cfg(test)]
mod unit_tests;

use crate::ICheckedOps;

pub trait IPanickingOps<T = Self>: Sized {
    type Output/*: Into<()>*/;

    #[must_use]
    fn panicking_abs(self) -> Self::Output;
    #[must_use]
    fn panicking_add(self, rhs: T) -> Self::Output;
    #[must_use]
    fn panicking_div(self, rhs: T) -> Self::Output;
    #[must_use]
    fn panicking_div_euclid(self, rhs: T) -> Self::Output;
    #[must_use]
    fn panicking_mul(self, rhs: T) -> Self::Output;
    #[must_use]
    fn panicking_neg(self) -> Self::Output;
    #[must_use]
    fn panicking_pow(self, rhs: u32) -> Self::Output;
    #[must_use]
    fn panicking_rem(self, rhs: T) -> Self::Output;
    #[must_use]
    fn panicking_rem_euclid(self, rhs: T) -> Self::Output;
    #[must_use]
    fn panicking_shl(self, rhs: u32) -> Self::Output;
    #[must_use]
    fn panicking_shr(self, rhs: u32) -> Self::Output;
    #[must_use]
    fn panicking_sub(self, rhs: T) -> Self::Output;
}

macro_rules! panicking_impl {
    ($tr:ty, $ret:ty; $($typ:ty),+ $(,)?) => ($(
        impl IPanickingOps for $typ where $typ: ICheckedOps {
            type Output = $ret;

            panicking_binary_op_impl! {
                $tr, $typ, $ret;
                panicking_add, checked_add,
                panicking_div, checked_div,
                panicking_div_euclid, checked_div_euclid,
                panicking_mul, checked_mul,
                panicking_rem, checked_rem,
                panicking_rem_euclid, checked_rem_euclid,
                panicking_sub, checked_sub
            }

            panicking_binary_op_impl! {
                $tr, u32, $ret;
                panicking_pow, checked_pow,
                panicking_shl, checked_shl,
                panicking_shr, checked_shr
            }

            panicking_unary_op_impl! {
                $ret;
                panicking_abs, checked_abs,
                panicking_neg, checked_neg
            }
        }
    )*)
}

panicking_impl! { IPanickingOps, Self; i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, }
