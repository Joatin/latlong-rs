use crate::Float;

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TangentPosition<T: Float> {
    pub x: T,
    pub y: T
}
