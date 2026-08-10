use crate::dt::release::ReleaseDt;
use crate::traits::{Packet, Q, Request};

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, PartialEq)]
pub struct ReleaseQ {}

impl Packet for ReleaseQ {
    const PKT_TYPE: u16 = 605;
}

impl Request for ReleaseQ {
    type R = ReleaseDt;
}

impl Q for ReleaseQ {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_ok() {
        assert_eq!("$PMTK605*31\r\n", ReleaseQ {}.serialize().unwrap());
    }
}