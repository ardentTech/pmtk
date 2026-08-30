use crate::packet::PktType;
use crate::traits::{Packet, CmdQ};

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Copy, Clone)]
pub struct LocusStatus {}

impl Packet for LocusStatus {
    const PKT_TYPE: PktType = [49, 56, 51]; // 183
}

impl CmdQ for LocusStatus {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_ok() {
        assert_eq!("$PMTK183*38\r\n", LocusStatus {}.serialize().unwrap());
    }
}