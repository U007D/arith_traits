#[cfg(test)]
mod unit_tests;

use crate::traits::IWrappingNonGenericOps;
use crate::IWrappingOps;
use num::{BigInt, Integer, Signed};
use std::ops::{Add, Div, Mul, Neg, Rem, Shl, Shr, Sub};

/// `BigInt` gets an `IWrapping` impl because it is the default signed extra-large type when the largest built-in type
/// is used in certain circumstances (e.g. `(i*::MIN..=i*::MAX).count() == i*::MAX + 1`).  Even when `i256` comes along,
/// this problem will remain, and `BigInt` will remain the default solution.
impl IWrappingOps for BigInt {
    fn wrapping_add(self, rhs: Self) -> <Self as IWrappingNonGenericOps>::Output {
        self.add(rhs)
    }

    fn wrapping_div(self, rhs: Self) -> <Self as IWrappingNonGenericOps>::Output {
        self.div(rhs)
    }

    fn wrapping_div_euclid(self, rhs: Self) -> <Self as IWrappingNonGenericOps>::Output {
        // Algorithm adapted from https://www.microsoft.com/en-us/research/wp-content/uploads/2016/02/divmodnote-letter.pdf
        let (quotient, remainder) = self.div_rem(&rhs);

        match remainder.is_negative() {
            true => match rhs.is_negative() {
                true => quotient + 1,
                false => quotient - 1,
            },
            false => quotient,
        }
    }

    fn wrapping_mul(self, rhs: Self) -> <Self as IWrappingNonGenericOps>::Output {
        self.mul(rhs)
    }

    fn wrapping_rem(self, rhs: Self) -> <Self as IWrappingNonGenericOps>::Output {
        self.rem(rhs)
    }

    fn wrapping_rem_euclid(self, rhs: Self) -> <Self as IWrappingNonGenericOps>::Output {
        // Algorithm adapted from https://www.microsoft.com/en-us/research/wp-content/uploads/2016/02/divmodnote-letter.pdf
        let (_, remainder) = self.div_rem(&rhs);
        match remainder.is_negative() {
            true => match rhs.is_negative() {
                true => remainder - rhs,
                false => remainder + rhs,
            },
            false => remainder,
        }
    }

    fn wrapping_sub(self, rhs: Self) -> <Self as IWrappingNonGenericOps>::Output {
        self.sub(rhs)
    }
}

impl IWrappingNonGenericOps for BigInt {
    type Output = Self;

    fn wrapping_abs(self) -> Self::Output {
        self.abs()
    }

    fn wrapping_neg(self) -> Self::Output {
        self.neg()
    }

    fn wrapping_pow(self, rhs: u32) -> Self::Output {
        self.pow(rhs)
    }

    fn wrapping_shl(self, rhs: u32) -> Self::Output {
        self.shl(rhs)
    }

    fn wrapping_shr(self, rhs: u32) -> Self::Output {
        self.shr(rhs)
    }
}
