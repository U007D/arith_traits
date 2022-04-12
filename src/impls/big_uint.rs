#[cfg(test)]
mod unit_tests;

use crate::traits::IWrappingNonGenericOps;
use crate::IWrappingOps;
use num::{BigUint, Integer, Zero};
use std::ops::{Add, Div, Mul, Rem, Shl, Shr, Sub};

/// `Big"Uint` gets an `IWrapping` impl because it is the default unsigned extra-large type when the largest built-in
/// type is used in certain circumstances (e.g. `(u*::MIN..=u*::MAX).count() == u*::MAX + 1`).  Even when `u256` comes
/// along,this problem will remain, and `BigUint` will remain the default solution.
impl IWrappingOps for BigUint {
    fn wrapping_add(self, rhs: Self) -> <Self as IWrappingNonGenericOps>::Output {
        self.add(rhs)
    }

    fn wrapping_div(self, rhs: Self) -> <Self as IWrappingNonGenericOps>::Output {
        self.div(rhs)
    }

    fn wrapping_div_euclid(self, rhs: Self) -> <Self as IWrappingNonGenericOps>::Output {
        self.div_rem(&rhs).0
    }

    fn wrapping_mul(self, rhs: Self) -> <Self as IWrappingNonGenericOps>::Output {
        self.mul(rhs)
    }

    fn wrapping_rem(self, rhs: Self) -> <Self as IWrappingNonGenericOps>::Output {
        self.rem(rhs)
    }

    fn wrapping_rem_euclid(self, rhs: Self) -> <Self as IWrappingNonGenericOps>::Output {
        self.div_rem(&rhs).1
    }

    fn wrapping_sub(self, rhs: Self) -> <Self as IWrappingNonGenericOps>::Output {
        self.sub(rhs)
    }
}

impl IWrappingNonGenericOps for BigUint {
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
