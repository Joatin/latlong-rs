use core::ops::{Div, Mul, Sub};
use crate::Float;
use core::fmt;

/// Right Ascension
#[derive(Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct RightAscension<T: Float> {
    radians: T,
}

impl<T: Float> RightAscension<T> {
    pub fn from_radians(radians: T) -> Self {
        Self { radians }
    }

    pub fn from_degrees(deg: T) -> Self {
        Self::from_radians(deg.to_radians())
    }

    pub fn radians(self) -> T {
        self.radians
    }

    pub fn degrees(self) -> T {
        self.radians.to_degrees()
    }

    pub fn sin(&self) -> T {
        self.radians.sin()
    }

    pub fn cos(&self) -> T {
        self.radians.cos()
    }

    pub fn is_nan(&self) -> bool {
        self.radians.is_nan()
    }
}

impl<T: Float> fmt::Debug for RightAscension<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl<T: Float> fmt::Display for RightAscension<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let total_hours = self.degrees() / T::from(15.0);
        let h = total_hours.floor();
        let m = ((total_hours - h) * T::from(60.0)).floor();
        let s = (total_hours - h - m / T::from(60.0)) * T::from(3600.0);

        write!(f, "{:02}h {:02}m {:05.2}s", h, m, s)
    }
}

impl<T: Float> Sub for RightAscension<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            radians: self.radians - rhs.radians,
        }
    }
}

impl<T: Float> Div<T> for RightAscension<T> {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Self {
            radians: self.radians / rhs,
        }
    }
}

impl<T: Float> Mul<T> for RightAscension<T> {
    type Output = RightAscension<T>;

    fn mul(self, rhs: T) -> Self::Output {
        RightAscension::from_radians(self.radians * rhs)
    }
}

const PRECISION: f32 = 100_000.0;

impl<T: Float> PartialEq for RightAscension<T> {
    fn eq(&self, other: &RightAscension<T>) -> bool {
        (self.radians * T::from(PRECISION)).round() == (other.radians * T::from(PRECISION)).round()
    }
}
