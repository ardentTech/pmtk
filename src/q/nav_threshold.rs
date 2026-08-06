use crate::dt::nav_threshold;
use crate::dt::nav_threshold::NavThresholdDt;
use crate::traits::{PmtkSentence, PmtkQ, PmtkBiDir};

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, PartialEq)]
pub struct NavThresholdQ {}

impl PmtkSentence for NavThresholdQ {
    const PKT_TYPE: u16 = 447;
}

impl PmtkBiDir for NavThresholdQ {
    type Dt = NavThresholdDt;
}

impl PmtkQ for NavThresholdQ {}

#[cfg(test)]
mod tests {
    use crate::packet::PmtkPacket;
    use super::*;

    #[test]
    fn encode_ok() {
        let query = NavThresholdQ {};
        let packet = PmtkPacket {
            checksum: 0x35,
            data_field: None,
            pkt_type: NavThresholdQ::PKT_TYPE,
        };
        assert_eq!(packet, query.marshal().unwrap());
    }
}