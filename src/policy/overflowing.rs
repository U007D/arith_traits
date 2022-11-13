use core::{fmt::{Debug, Display}, ops::Add};
use crate::IOverflowingOps;

pub struct OverflowingPolicy<T>(pub T, pub bool);

impl<T: IOverflowingOps<T>> Add for OverflowingPolicy<T> {
    type Output = Self;

    #[allow(clippy::suspicious_arithmetic_impl)]
    fn add(self, rhs: Self) -> Self::Output {
        let (value, mut overflow) = self.0.overflowing_add(rhs.0);
        overflow |= self.1 | rhs.1;
        (value, overflow).into()
    }
} 

impl<T: IOverflowingOps<T>> Add<T> for OverflowingPolicy<T> {
    type Output = Self;

    #[allow(clippy::suspicious_arithmetic_impl)]
    fn add(self, rhs: T) -> Self::Output {
        let (value, mut overflow) = self.0.overflowing_add(rhs);
        overflow |= self.1;
        (value, overflow).into()
    }
}

impl<T: Debug> Debug for OverflowingPolicy<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Overflowing").field(&self.0).field(&self.1).finish()
    }
} 

impl<T: Display> Display for OverflowingPolicy<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

impl<T> From<(T, bool)> for OverflowingPolicy<T> {
    fn from((value, overflow): (T, bool)) -> Self {
        Self(value, overflow)
    }
}
