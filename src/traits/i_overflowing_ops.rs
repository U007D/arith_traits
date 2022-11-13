// suppress `use_self` recommendation; unavoidable in macro context
#![allow(clippy::use_self)]

#[cfg(test)]
mod unit_tests;

pub trait IOverflowingOps<T>: Sized {
    #[must_use]
    fn overflowing_abs(self) -> (Self, bool);
    #[must_use]
    fn overflowing_add(self, rhs: T) -> (Self, bool);
    #[must_use]
    fn overflowing_div(self, rhs: T) -> (Self, bool);
    #[must_use]
    fn overflowing_div_euclid(self, rhs: T) -> (Self, bool);
    #[must_use]
    fn overflowing_mul(self, rhs: T) -> (Self, bool);
    #[must_use]
    fn overflowing_neg(self) -> (Self, bool);
    #[must_use]
    fn overflowing_pow(self, rhs: u32) -> (Self, bool);
    #[must_use]
    fn overflowing_rem(self, rhs: T) -> (Self, bool);
    #[must_use]
    fn overflowing_rem_euclid(self, rhs: T) -> (Self, bool);
    #[must_use]
    fn overflowing_shl(self, rhs: u32) -> (Self, bool);
    #[must_use]
    fn overflowing_shr(self, rhs: u32) -> (Self, bool);
    #[must_use]
    fn overflowing_sub(self, rhs: T) -> (Self, bool);
}

macro_rules! overflowing_impl {
    ($tr:ty, $ret:ty; $($typ:ty),+ $(,)?) => ($(
        impl IOverflowingOps<Self> for $typ {
            binary_op_impl! {
                $tr, $typ, $ret;
                overflowing_add,
                overflowing_div,
                overflowing_div_euclid,
                overflowing_mul,
                overflowing_rem,
                overflowing_rem_euclid,
                overflowing_sub
            }

            binary_op_impl! {
                $tr, u32, $ret;
                overflowing_pow,
                overflowing_shl,
                overflowing_shr
            }

            unary_op_impl! {
                $ret;
                overflowing_abs,
                overflowing_neg
            }
        }
    )*)
}

overflowing_impl! { IOverflowingOps, (Self, bool); i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, }
