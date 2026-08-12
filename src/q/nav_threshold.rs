use crate::traits::{Packet, Q, Request};

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, PartialEq)]
pub struct NavThresholdQ {}

impl Packet for NavThresholdQ {
    const PKT_TYPE: u16 = 447;
}

impl Request for NavThresholdQ {}

impl Q for NavThresholdQ {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_ok() {
        assert_eq!("$PMTK447*35\r\n", NavThresholdQ {}.serialize().unwrap());
    }
}