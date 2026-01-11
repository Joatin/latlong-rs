use std::fmt;
use std::ops::{Div, Sub};
use num::{Float, NumCast};

/// Declination
#[derive(Clone, Copy, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
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

impl<T: Float + std::fmt::Display> fmt::Debug for Declination<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self
        )
    }
}

impl<T: Float + std::fmt::Display> fmt::Display for Declination<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let deg = self.degrees();
        let sign = if deg < NumCast::from(0.0).unwrap() { '-' } else { '+' };
        let abs = deg.abs();

        let d = abs.floor();
        let m = ((abs - d) * NumCast::from(60.0).unwrap()).floor();
        let s = (abs - d - m / NumCast::from(60.0).unwrap()) * NumCast::from(3600.0).unwrap();

        write!(
            f,
            "{}{:02}° {:02}' {:05.2}\"",
            sign,
            d.to_i32().unwrap(),
            m.to_i32().unwrap(),
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
        (self.radians * NumCast::from(PRECISION).unwrap()).round() == (other.radians * NumCast::from(PRECISION).unwrap()).round()
    }
}
