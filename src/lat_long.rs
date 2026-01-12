use crate::Float;
use crate::latitude::Latitude;
use crate::longitude::Longitude;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LatLong<T: Float> {
    pub latitude: Latitude<T>,
    pub longitude: Longitude<T>,
}