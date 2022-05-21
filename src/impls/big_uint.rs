#[cfg(test)]
mod unit_tests;

use crate::traits::IUnaryWrappingOps;
use crate::IWrappingOps;
use num::{BigUint, Integer, Zero};
use std::ops::{Add, Div, Mul, Rem, Shl, Shr, Sub};

/// `Big"Uint` gets an `IWrapping` impl because it is the default unsigned extra-large type when the largest built-in
/// type is used in certain circumstances (e.g. `(u*::MIN..=u*::MAX).count() == u*::MAX + 1`).  Even once `u256` comes
/// along,this problem will remain (for `u256`), and `BigUint` will remain the default solution.
impl IWrappingOps for BigUint {
    fn wrapping_add(self, rhs: Self) -> Self::AddOutput {
        self.add(rhs)
    }

    fn wrapping_div(self, rhs: Self) -> Self::DivOutput {
        self.div(rhs)
    }

    fn wrapping_div_euclid(self, rhs: Self) -> Self::DivOutput {
        self.div_rem(&rhs).0
    }

    fn wrapping_mul(self, rhs: Self) -> Self::MulOutput {
        self.mul(rhs)
    }

    fn wrapping_rem(self, rhs: Self) -> <Self as IWrappingOps>::Output {
        self.rem(rhs)
    }

    fn wrapping_rem_euclid(self, rhs: Self) -> <Self as IWrappingOps>::Output {
        self.div_rem(&rhs).1
    }

    fn wrapping_sub(self, rhs: Self) -> Self::SubOutput {
        self.sub(rhs)
    }
}

impl<'a> IWrappingOps for &'a BigUint {
    type AddOutput = BigUint;
    type DivOutput = BigUint;
    type MulOutput = BigUint;
    type Output = BigUint;
    type SubOutput = BigUint;

    fn wrapping_add(self, rhs_ref: Self) -> Self::AddOutput {
        self.add(rhs_ref)
    }

    fn wrapping_div(self, rhs_ref: Self) -> Self::DivOutput {
        self.div(rhs_ref)
    }

    fn wrapping_div_euclid(self, rhs_ref: Self) -> Self::DivOutput {
        self.div_rem(rhs_ref).0
    }

    fn wrapping_mul(self, rhs_ref: Self) -> Self::MulOutput {
        self.mul(rhs_ref)
    }

    fn wrapping_rem(self, rhs_ref: Self) -> <Self as IWrappingOps>::Output {
        self.rem(rhs_ref)
    }

    fn wrapping_rem_euclid(self, rhs_ref: Self) -> <Self as IWrappingOps>::Output {
        self.div_rem(rhs_ref).1
    }

    fn wrapping_sub(self, rhs_ref: Self) -> Self::SubOutput {
        self.sub(rhs_ref)
    }
}

impl IUnaryWrappingOps for BigUint {
    type Output = Self;

    fn wrapping_abs(self) -> Self::Output {
        self
    }

    fn wrapping_neg(self) -> Self::Output {
        Self::zero().wrapping_sub(self)
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

impl<'a> IUnaryWrappingOps for &'a BigUint {
    type Output = BigUint;

    fn wrapping_abs(self) -> Self::Output {
        self.clone()
    }

    fn wrapping_neg(self) -> Self::Output {
        (&BigUint::zero()).wrapping_sub(self)
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
