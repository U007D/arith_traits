#[cfg(test)]
mod unit_tests;

use crate::traits::IUnaryWrappingOps;
use crate::IWrappingOps;
use num::{BigInt, Integer, Signed};
use std::ops::{Add, Div, Mul, Neg, Rem, Shl, Shr, Sub};

/// `BigInt` gets an `IWrappingOps` impl because it is the default signed extra-large type when the
/// largest built-in type is used in certain circumstances (e.g.
/// `(i*::MIN..=i*::MAX).count() == i*::MAX + 1`).  Even once `i256` comes along, this problem will
/// remain (for `i256`), and `BigInt` will remain the default solution.
impl IWrappingOps for BigInt {
    fn wrapping_add(self, rhs: Self) -> Self::AddOutput {
        self.add(rhs)
    }

    fn wrapping_div(self, rhs: Self) -> Self::DivOutput {
        self.div(rhs)
    }

    fn wrapping_div_euclid(self, rhs: Self) -> Self::DivOutput {
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

    fn wrapping_mul(self, rhs: Self) -> Self::MulOutput {
        self.mul(rhs)
    }

    fn wrapping_rem(self, rhs: Self) -> Self {
        self.rem(rhs)
    }

    fn wrapping_rem_euclid(self, rhs: Self) -> Self {
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

    fn wrapping_sub(self, rhs: Self) -> Self::SubOutput {
        self.sub(rhs)
    }
}

impl<'a> IWrappingOps for &'a BigInt {
    type AddOutput = BigInt;
    type DivOutput = BigInt;
    type MulOutput = BigInt;
    type RemOutput = BigInt;
    type SubOutput = BigInt;

    fn wrapping_add(self, rhs_ref: Self) -> Self::AddOutput {
        self.add(rhs_ref)
    }

    fn wrapping_div(self, rhs_ref: Self) -> Self::DivOutput {
        self.div(rhs_ref)
    }

    fn wrapping_div_euclid(self, rhs_ref: Self) -> Self::DivOutput {
        // Algorithm adapted from https://www.microsoft.com/en-us/research/wp-content/uploads/2016/02/divmodnote-letter.pdf
        let (quotient, remainder) = self.div_rem(rhs_ref);

        match remainder.is_negative() {
            true => match rhs_ref.is_negative() {
                true => quotient + 1,
                false => quotient - 1,
            },
            false => quotient,
        }
    }

    fn wrapping_mul(self, rhs_ref: Self) -> Self::MulOutput {
        self.mul(rhs_ref)
    }

    fn wrapping_rem(self, rhs_ref: Self) -> Self::RemOutput {
        self.rem(rhs_ref)
    }

    fn wrapping_rem_euclid(self, rhs_ref: Self) -> Self::RemOutput {
        // Algorithm adapted from https://www.microsoft.com/en-us/research/wp-content/uploads/2016/02/divmodnote-letter.pdf
        let (_, remainder) = self.div_rem(rhs_ref);
        match remainder.is_negative() {
            true => match rhs_ref.is_negative() {
                true => remainder - rhs_ref,
                false => remainder + rhs_ref,
            },
            false => remainder,
        }
    }

    fn wrapping_sub(self, rhs_ref: Self) -> Self::SubOutput {
        self.sub(rhs_ref)
    }
}

impl IUnaryWrappingOps for BigInt {
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

impl<'a> IUnaryWrappingOps for &'a BigInt {
    type Output = BigInt;

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
