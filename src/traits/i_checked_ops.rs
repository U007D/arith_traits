// suppress `use_self` recommendation; not avoidable in macro context
#![allow(clippy::use_self)]

#[cfg(test)]
mod unit_tests;

pub trait ICheckedOps<T = Self>: Sized
where
    Self: PartialOrd,
{
    type Output;

    fn checked_abs(self) -> Self::Output;
    fn checked_add(self, rhs: T) -> Self::Output;
    fn checked_div(self, rhs: T) -> Self::Output;
    fn checked_div_euclid(self, rhs: T) -> Self::Output;
    fn checked_mul(self, rhs: T) -> Self::Output;
    fn checked_neg(self) -> Self::Output;
    fn checked_pow(self, rhs: u32) -> Self::Output;
    fn checked_rem(self, rhs: T) -> Self::Output;
    fn checked_rem_euclid(self, rhs: T) -> Self::Output;
    fn checked_shl(self, rhs: u32) -> Self::Output;
    fn checked_shr(self, rhs: u32) -> Self::Output;
    fn checked_sub(self, rhs: T) -> Self::Output;
}

macro_rules! checked_impl {
    ($($t:ty)*) => ($(
        impl ICheckedOps for $t {
            type Output = Option<Self>;

            binary_op_impl! {
                $t,
                checked_add,
                checked_div,
                checked_div_euclid,
                checked_mul,
                checked_rem,
                checked_rem_euclid,
                checked_sub
            }

            binary_op_impl! {
                u32,
                checked_pow,
                checked_shl,
                checked_shr
            }

            unary_op_impl! {
                checked_abs,
                checked_neg
            }
        }
    )*)
}

checked_impl! { i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize }
