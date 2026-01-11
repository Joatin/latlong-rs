use std::fmt;
use std::ops::{Div, Mul, Sub};
use num::{Float, NumCast};
use num::traits::Euclid;

/// Right Ascension
#[derive(Clone, Copy, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct RightAscension<T: Float> {
    radians: T,
}

impl<T: Float + Euclid> RightAscension<T> {
    pub fn from_radians(radians: T) -> Self {
        Self { radians: radians.rem_euclid(&NumCast::from(std::f64::consts::TAU).unwrap()) }
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

impl<T: Float + std::fmt::Display + Euclid> fmt::Debug for RightAscension<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl<T: Float + std::fmt::Display + Euclid> fmt::Display for RightAscension<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let total_hours = self.degrees() / NumCast::from(15.0).unwrap();
        let h = total_hours.floor();
        let m = ((total_hours - h) * NumCast::from(60.0).unwrap()).floor();
        let s = (total_hours - h - m / NumCast::from(60.0).unwrap()) * NumCast::from(3600.0).unwrap();

        write!(f, "{:02}h {:02}m {:05.2}s", h.to_i32().unwrap(), m.to_i32().unwrap(), s)
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

impl<T: Float + num::traits::Euclid> Mul<T> for RightAscension<T> {
    type Output = RightAscension<T>;

    fn mul(self, rhs: T) -> Self::Output {
        RightAscension::from_radians(self.radians * rhs)
    }
}

const PRECISION: f32 = 100_000.0;

impl<T: Float> PartialEq for RightAscension<T> {
    fn eq(&self, other: &RightAscension<T>) -> bool {
        (self.radians * NumCast::from(PRECISION).unwrap()).round() == (other.radians * NumCast::from(PRECISION).unwrap()).round()
    }
}
