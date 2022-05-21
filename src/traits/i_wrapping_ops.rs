// suppress spurious? `clippy::use_self` lint
#![allow(clippy::use_self)]

mod i_unary_wrapping_ops;
#[cfg(test)]
mod unit_tests;

pub use i_unary_wrapping_ops::IUnaryWrappingOps;

pub trait IWrappingOps<T = Self>: IUnaryWrappingOps {
    type AddOutput = Self;
    type DivOutput = Self;
    type MulOutput = Self;
    type Output = Self;
    type SubOutput = Self;

    fn wrapping_add(self, rhs: T) -> Self::AddOutput;
    fn wrapping_div(self, rhs: T) -> Self::DivOutput;
    fn wrapping_div_euclid(self, rhs: T) -> Self::DivOutput;
    fn wrapping_mul(self, rhs: T) -> Self::MulOutput;
    fn wrapping_rem(self, rhs: T) -> <Self as IWrappingOps>::Output where Self: IWrappingOps;
    fn wrapping_rem_euclid(self, rhs: T) -> <Self as IWrappingOps>::Output where Self: IWrappingOps;
    fn wrapping_sub(self, rhs: T) -> Self::SubOutput;
}

macro_rules! wrapping_ops {
    ($tr:ty; $($t:ty),+ $(,)?) => ($(
        impl IWrappingOps for $t {
            type Output = Self;

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

wrapping_ops! { IUnaryWrappingOps; i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, }
