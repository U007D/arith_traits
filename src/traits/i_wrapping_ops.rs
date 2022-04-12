// suppress spurious? `clippy::use_self` lint
#![allow(clippy::use_self)]

mod i_wrapping_non_generic_ops;
#[cfg(test)]
mod unit_tests;

pub use i_wrapping_non_generic_ops::IWrappingNonGenericOps;

pub trait IWrappingOps<T = Self>: IWrappingNonGenericOps {
    fn wrapping_add(self, rhs: T) -> <Self as IWrappingNonGenericOps>::Output;
    fn wrapping_div(self, rhs: T) -> <Self as IWrappingNonGenericOps>::Output;
    fn wrapping_div_euclid(self, rhs: T) -> <Self as IWrappingNonGenericOps>::Output;
    fn wrapping_mul(self, rhs: T) -> <Self as IWrappingNonGenericOps>::Output;
    fn wrapping_rem(self, rhs: T) -> <Self as IWrappingNonGenericOps>::Output;
    fn wrapping_rem_euclid(self, rhs: T) -> <Self as IWrappingNonGenericOps>::Output;
    fn wrapping_sub(self, rhs: T) -> <Self as IWrappingNonGenericOps>::Output;
}

macro_rules! wrapping_ops {
    ($tr:ty; $($t:ty),+ $(,)?) => ($(
        impl IWrappingOps for $t {
            binary_op_impl! {
                $tr, $t;
                wrapping_add,
                wrapping_div,
                wrapping_div_euclid,
                wrapping_mul,
                wrapping_rem,
                wrapping_rem_euclid,
                wrapping_sub,
            }
        }
    )*)
}

wrapping_ops! { IWrappingNonGenericOps; i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, }
