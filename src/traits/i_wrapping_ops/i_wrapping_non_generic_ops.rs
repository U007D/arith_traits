pub trait IWrappingNonGenericOps {
    type Output;

    fn wrapping_abs(self) -> Self::Output;
    fn wrapping_neg(self) -> Self::Output;
    fn wrapping_pow(self, rhs: u32) -> Self::Output;
    fn wrapping_shl(self, rhs: u32) -> Self::Output;
    fn wrapping_shr(self, rhs: u32) -> Self::Output;
}

macro_rules! wrapping_non_generic_ops {
    ($tr:ty; $($t:ty),+ $(,)?) => ($(
        impl $tr for $t {
            type Output = Self;

            binary_op_impl! {
                $tr, u32;
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

wrapping_non_generic_ops! { IWrappingNonGenericOps; i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, }
