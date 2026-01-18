use core::ops::Div;
use core::fmt::{Debug, Display};
use core::ops::Mul;
use core::ops::Sub;
use std::ops::Add;

/// We do not use num crate since it is not no_std
pub trait Float: Into<Self> + Copy + Debug + Div<Self, Output = Self> + Mul<Self, Output = Self> + Sub<Self, Output = Self> + Add<Self, Output = Self>  + PartialEq + PartialOrd + Display {

    fn floor(self) -> Self;
    fn is_nan(self) -> bool;
    fn to_radians(self) -> Self;
    fn to_degrees(self) -> Self;
    fn sin(self) -> Self;
    fn cos(self) -> Self;
    fn round(self) -> Self;
    fn abs(self) -> Self;

    fn to_f64(self) -> f64;
    fn to_f32(self) -> f32;

    fn from<T: Float>(f: T) -> Self;
}

impl Float for f64 {

    fn floor(self) -> Self {
        f64::floor(self)
    }

    fn is_nan(self) -> bool {
        f64::is_nan(self)
    }

    fn to_radians(self) -> Self {
        f64::to_radians(self)
    }

    fn to_degrees(self) -> Self {
        f64::to_degrees(self)
    }

    fn sin(self) -> Self {
        f64::sin(self)
    }

    fn cos(self) -> Self {
        f64::cos(self)
    }

    fn round(self) -> Self {
        f64::round(self)
    }

    fn abs(self) -> Self {
        f64::abs(self)
    }

    fn to_f64(self) -> f64 {
        self
    }

    fn to_f32(self) -> f32 {
        self as f32
    }

    fn from<T: Float>(f: T) -> Self {
        f.to_f64()
    }
}

impl Float for f32 {

    fn floor(self) -> Self {
        f32::floor(self)
    }

    fn is_nan(self) -> bool {
        f32::is_nan(self)
    }

    fn to_radians(self) -> Self {
        f32::to_radians(self)
    }

    fn to_degrees(self) -> Self {
        f32::to_degrees(self)
    }

    fn sin(self) -> Self {
        f32::sin(self)
    }

    fn cos(self) -> Self {
        f32::cos(self)
    }

    fn round(self) -> Self {
        f32::round(self)
    }

    fn abs(self) -> Self {
        f32::abs(self)
    }

    fn to_f64(self) -> f64 {
        self as f64
    }

    fn to_f32(self) -> f32 {
        self
    }

    fn from<T: Float>(f: T) -> Self {
        f.to_f32()
    }
}