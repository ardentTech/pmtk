use crate::packet::PktType;
use crate::traits::{Packet, CmdQ};

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, PartialEq)]
pub struct NavThresholdQ {}

impl Packet for NavThresholdQ {
    const PKT_TYPE: PktType = [52, 52, 55]; // 447
}

impl CmdQ for NavThresholdQ {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_ok() {
        assert_eq!("$PMTK447*35\r\n", NavThresholdQ {}.serialize().unwrap());
    }
}