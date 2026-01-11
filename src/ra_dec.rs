use crate::{Declination, Float, RightAscension};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RaDec<T: Float> {
    pub ra: RightAscension<T>,
    pub dec: Declination<T>
}

