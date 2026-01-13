use std::ops::{Mul, Sub};
use crate::{Declination, Float, RightAscension};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RaDec<T: Float> {
    pub ra: RightAscension<T>,
    pub dec: Declination<T>
}

impl<T: Float> Sub<RaDec<T>> for RaDec<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            ra: self.ra - rhs.ra,
            dec: self.dec - rhs.dec,
        }
    }
}

impl<T: Float> Mul<T> for RaDec<T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self {
            ra: self.ra * rhs,
            dec: self.dec * rhs,
        }
    }
}

