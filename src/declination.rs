use core::ops::{Div, Sub};
use crate::Float;
use core::fmt;

/// Declination
#[derive(Clone, Copy, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct Declination<T: Float> {
    radians: T,
}

impl<T: Float> Declination<T> {
    pub fn from_radians(radians: T) -> Self {
        Self { radians }
    }

    pub fn from_degrees(degrees: T) -> Self {
        Self {
            radians: degrees.to_radians()
        }
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

impl<T: Float> fmt::Debug for Declination<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self
        )
    }
}

impl<T: Float> fmt::Display for Declination<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let deg = self.degrees();
        let sign = if deg < T::from(0.0) { '-' } else { '+' };
        let abs = deg.abs();

        let d = abs.floor();
        let m = ((abs - d) * T::from(60.0)).floor();
        let s = (abs - d - m / T::from(60.0)) * T::from(3600.0);

        write!(
            f,
            "{}{:02}° {:02}' {:05.2}\"",
            sign,
            d,
            m,
            s
        )
    }
}

impl<T: Float> Sub for Declination<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            radians: self.radians - rhs.radians,
        }
    }
}

impl<T: Float> Div<T> for Declination<T> {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Self {
            radians: self.radians / rhs,
        }
    }
}

const PRECISION: f64 = 100_000.0;

impl<T: Float> PartialEq for Declination<T> {
    fn eq(&self, other: &Self) -> bool {
        (self.radians * T::from(PRECISION)).round() == (other.radians * T::from(PRECISION)).round()
    }
}
