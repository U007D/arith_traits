// suppress `use_self` recommendation; unavoidable in macro context
#![allow(clippy::use_self)]

#[cfg(test)]
mod unit_tests;

use crate::ICheckedOps;

pub trait IPanickingOps<T = Self>
where
    Self: PartialOrd,
{
    type Output;

    fn panicking_abs(self) -> Self::Output;
    fn panicking_add(self, rhs: T) -> Self::Output;
    fn panicking_div(self, rhs: T) -> Self::Output;
    fn panicking_div_euclid(self, rhs: T) -> Self::Output;
    fn panicking_mul(self, rhs: T) -> Self::Output;
    fn panicking_neg(self) -> Self::Output;
    fn panicking_pow(self, rhs: u32) -> Self::Output;
    fn panicking_rem(self, rhs: T) -> Self::Output;
    fn panicking_rem_euclid(self, rhs: T) -> Self::Output;
    fn panicking_shl(self, rhs: u32) -> Self::Output;
    fn panicking_shr(self, rhs: u32) -> Self::Output;
    fn panicking_sub(self, rhs: T) -> Self::Output;
}

macro_rules! panicking_impl {
    ($($t:ty)*) => ($(
        impl IPanickingOps for $t where $t: ICheckedOps {
            type Output = Self;

            panicking_binary_op_impl! {
                $t,
                panicking_add, checked_add,
                panicking_div, checked_div,
                panicking_div_euclid, checked_div_euclid,
                panicking_mul, checked_mul,
                panicking_rem, checked_rem,
                panicking_rem_euclid, checked_rem_euclid,
                panicking_sub, checked_sub
            }

            panicking_binary_op_impl! {
                u32,
                panicking_pow, checked_pow,
                panicking_shl, checked_shl,
                panicking_shr, checked_shr
            }

            panicking_unary_op_impl! {
                panicking_abs, checked_abs,
                panicking_neg, checked_neg
            }
        }
    )*)
}

panicking_impl! { i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize }
