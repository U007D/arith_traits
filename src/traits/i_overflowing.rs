// suppress `use_self` recommendation; unavoidable in macro context
#![allow(clippy::use_self)]

#[cfg(test)]
mod unit_tests;

pub trait IOverflowing<T = Self> where Self: PartialOrd {
    type Output;

    fn overflowing_abs(self) -> Self::Output;
    fn overflowing_add(self, rhs: T) -> Self::Output;
    fn overflowing_div(self, rhs: T) -> Self::Output;
    fn overflowing_div_euclid(self, rhs: T) -> Self::Output;
    fn overflowing_mul(self, rhs: T) -> Self::Output;
    fn overflowing_neg(self) -> Self::Output;
    fn overflowing_pow(self, rhs: u32) -> Self::Output;
    fn overflowing_rem(self, rhs: T) -> Self::Output;
    fn overflowing_rem_euclid(self, rhs: T) -> Self::Output;
    fn overflowing_shl(self, rhs: u32) -> Self::Output;
    fn overflowing_shr(self, rhs: u32) -> Self::Output;
    fn overflowing_sub(self, rhs: T) -> Self::Output;
}

macro_rules! overflowing_impl {
    ($($t:ty)*) => ($(
        impl IOverflowing for $t {
            type Output = (Self, bool);

            binary_op_impl! {
                $t,
                overflowing_add,
                overflowing_div,
                overflowing_div_euclid,
                overflowing_mul,
                overflowing_rem,
                overflowing_rem_euclid,
                overflowing_sub
            }

            binary_op_impl! {
                u32,
                overflowing_pow,
                overflowing_shl,
                overflowing_shr
            }

            unary_op_impl! {
                overflowing_abs,
                overflowing_neg
            }
        }
    )*)
}

overflowing_impl! { i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize }
