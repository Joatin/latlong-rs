use num::Float;
use crate::latitude::Latitude;
use crate::longitude::Longitude;

pub struct LatLong<T: Float> {
    latitude: Latitude<T>,
    longitude: Longitude<T>,
}