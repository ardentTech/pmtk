use crate::dt::nav_threshold;
use crate::traits::{Message, Query};

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, PartialEq)]
pub struct NavThresholdQ {}

impl Message for NavThresholdQ {
    const PKT_TYPE: u16 = 447;
}

impl Query for NavThresholdQ {
    type R = nav_threshold::NavThresholdDt;
}

#[cfg(test)]
mod tests {
    use crate::types::PmtkPacket;
    use super::*;

    #[test]
    fn encode_ok() {
        let query = NavThresholdQ {};
        let packet = PmtkPacket {
            checksum: 0x35,
            data_field: None,
            pkt_type: NavThresholdQ::PKT_TYPE,
        };
        assert_eq!(packet, query.encode().unwrap());
    }
}