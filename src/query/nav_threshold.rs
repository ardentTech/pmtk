use crate::response::nav_threshold;
use crate::traits::{Message, Query};

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, PartialEq)]
pub struct NavThreshold {}

impl Message for NavThreshold {
    const PKT_TYPE: u16 = 447;
}

impl Query for NavThreshold {
    type Response = nav_threshold::NavThreshold;
}