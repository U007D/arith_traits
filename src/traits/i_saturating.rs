// suppress `use_self` recommendation; unavoidable in macro context
#![allow(clippy::use_self)]

#[cfg(test)]
mod unit_tests;

pub trait ISaturating<T = Self> where Self: PartialOrd {
    type Output;

    fn saturating_abs(self) -> Self::Output;
    fn saturating_add(self, rhs: T) -> Self::Output;
    fn saturating_div(self, rhs: T) -> Self::Output;
    fn saturating_div_euclid(self, rhs: T) -> Self::Output;
    fn saturating_mul(self, rhs: T) -> Self::Output;
    fn saturating_neg(self) -> Self::Output;
    fn saturating_pow(self, rhs: u32) -> Self::Output;
    fn saturating_rem(self, rhs: T) -> Self::Output;
    fn saturating_rem_euclid(self, rhs: T) -> Self::Output;
    fn saturating_shl(self, rhs: u32) -> Self::Output;
    fn saturating_shr(self, rhs: u32) -> Self::Output;
    fn saturating_sub(self, rhs: T) -> Self::Output;
}

macro_rules! saturating_impl {
    ($($t:ty)*) => ($(
        impl ISaturating for $t {
            type Output = Self;

            binary_op_impl! {
                $t,
                saturating_add,
                saturating_div,
                saturating_div_euclid,
                saturating_mul,
                saturating_rem,
                saturating_rem_euclid,
                saturating_sub
            }

            binary_op_impl! {
                u32,
                saturating_pow,
                saturating_shl,
                saturating_shr
            }

            unary_op_impl! {
                saturating_abs,
                saturating_neg
            }
        }
    )*)
}

saturating_impl! { i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize }
