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

#[cfg(test)]
mod tests {
    use crate::types::PmtkPacket;
    use super::*;

    #[test]
    fn encode_ok() {
        let query = NavThreshold {};
        let packet = PmtkPacket {
            checksum: 0x35,
            data_field: None,
            pkt_type: NavThreshold::PKT_TYPE,
        };
        assert_eq!(packet, query.encode().unwrap());
    }
}