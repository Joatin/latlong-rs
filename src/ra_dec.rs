use num::Float;
use crate::{Declination, RightAscension};

pub struct RaDec<T: Float> {
    pub ra: RightAscension<T>,
    pub dec: Declination<T>
}

